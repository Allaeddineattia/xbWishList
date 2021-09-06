use crate::service::wishlist_service::WishlistService;
use std::sync::Arc;
use super::dto;
use super::dto::input::UpdateWishlistPreferenceDTO;
use actix_web::{Responder, web, HttpResponse, Scope};
use std::collections::{HashSet, HashMap};
use std::borrow::Borrow;
use crate::service::game_service::GameService;
use crate::core::wishlist::{Markets, Wishlist, WishlistElement};

pub struct WishlistController{
    wishlist_service: Arc<WishlistService>,
    game_service: Arc<GameService>
}

enum CreateUpdatedGameListResponse{
    Ok(HashMap<String, WishlistElement>),
    RedundantGameError(String),
    MissingGame(String),
    GameDosentBelongToWishlist(String),
}

impl WishlistController{
    pub fn new(wishlist_service: Arc<WishlistService>, game_service: Arc<GameService>) -> Self {
        WishlistController { wishlist_service, game_service }
    }

    // Post /create
    pub async fn create_wishlist(form: web::Json<dto::input::CreateWishlist>, data: web::Data<WishlistController>) -> impl Responder {
        let dto = form.into_inner();
        let mut preferred_markets = Markets::from_vec_str(dto.markets).0;
        let mut game_list = HashMap::<String, WishlistElement>::new();

        for game_dto in dto.games.into_iter(){
            let mut markets;
            if let Some(markets_list) = game_dto.markets{
                markets = Markets::from_vec_str(markets_list).0;
            }else{
                markets = preferred_markets.clone();
            }
            let game = data.game_service.get_game_info(&game_dto.id, &dto.language, markets.to_vec()).await;
            if let Some(game) = game{
                if let Some(_) = game_list.insert(game.id().to_string(),WishlistElement::new(game,markets)){
                    let error_message = "game with id ".to_string() + &game_dto.id +
                        " is redundant please make sure it's present only once";
                    return HttpResponse::BadRequest()
                        .body(&error_message);
                }
            }else{
                let error_message = "couldn't get game with id ".to_string() + &game_dto.id;
                return HttpResponse::BadRequest()
                    .body(&error_message);
            }
        };

        let wishlist_pref = crate::core::wishlist::WishlistPreferences{
            language: dto.language,
            markets_by_default: preferred_markets
        };

        let wishlist = crate::core::wishlist::Wishlist::new(dto.name , wishlist_pref, game_list);
        data.wishlist_service.save(&wishlist).await;
        if let Some(wishlist) = data.wishlist_service.get_wishlist(&wishlist.name).await{
            return HttpResponse::Created()
            .content_type("application/json")
            .json(
                Self::entity_to_dto(wishlist)
            )
        }else {
            return HttpResponse::InternalServerError()
            .json(
                "could not fetch wishlist "
            )
        }
    }

    async fn get_wishlist_games(&self, vec : Vec<(&str, Vec<&str>)>, language:&str)->Vec<dto::output::wishlist_info::WishlistInfoElement>{
        let mut result = Vec::<dto::output::wishlist_info::WishlistInfoElement>::new();
        for pair in vec.into_iter(){
            let game = self.game_service.get_game_info(pair.0, language, pair.1.iter().map(|s|{&s[..]}).collect()).await.unwrap();
            let game_info = dto::output::wishlist_info::WishlistInfoElement{
                game: dto::output::GameInfo::new(game),
                markets: pair.1.iter().map(|s|{s.to_string()}).collect()
            };
            result.push(game_info);
        };
        result
    }

    //get //{name}
    pub async fn get_wishlist(web::Path((name)): web::Path<(String)>, data: web::Data<WishlistController>)-> impl Responder{
        let result = data.wishlist_service.get_wishlist(&name).await;
        if let Some(wishlist) = result{
            return HttpResponse::Created()
            .content_type("application/json")
            .json(
                Self::entity_to_dto(wishlist)
            )
        }else {
            return HttpResponse::InternalServerError()
            .json(
                "could not fetch wishlist "
            )
        }

    }
    //patch /add
    pub async fn add_game_to_wishlist(form: web::Json<dto::input::AddToWishlistDTO>, data: web::Data<WishlistController>)-> impl Responder{
        let dto = form.into_inner();
        if let Some(mut wishlist) = data.wishlist_service.get_wishlist(&dto.name).await{
            let element = dto.game;
            let markets: Markets;
            if let Some(market_list) = element.markets{
                markets = Markets::from_vec_str(market_list).0;
            }else{
                markets = wishlist.preference.markets_by_default.clone();
            }
            let game = data.game_service.get_game_info(&element.id, &wishlist.preference.language, markets.to_vec()).await;
            if let Some(game) = game{
                wishlist.add_a_game(game, Some(markets));
                data.wishlist_service.save(&wishlist).await;

                if let Some(wishlist) = data.wishlist_service.get_wishlist(&wishlist.name).await{
                    return HttpResponse::Created()
                    .content_type("application/json")
                    .json(
                        Self::entity_to_dto(wishlist)
                    )
                }else {
                    return HttpResponse::InternalServerError()
                    .json(
                        "could not fetch wishlist "
                    )
                }
            }else {
                let error_message = "couldn't get game with id ".to_string() + &element.id;
                return HttpResponse::BadRequest()
                    .body(&error_message);
            }
        }else{
            let error_message = "couldn't get wishlist with id ".to_string() + &dto.name;
            return HttpResponse::BadRequest()
                .body(&error_message)
        };
    }

    fn entity_to_dto(wishlist: Wishlist)->dto::output::wishlist_info::WishlistInfo{
        let games: Vec<dto::output::wishlist_info::WishlistInfoElement> = wishlist.games.into_iter().map(|pair|{
            let markets = pair.1.markets().into_iter().map(|str|{str.to_string()}).collect();
            let game = pair.1.game;
            dto::output::wishlist_info::WishlistInfoElement{
                game: dto::output::GameInfo::new(game),
                markets: markets
            }
        }).collect();
        dto::output::wishlist_info::WishlistInfo{
            name: wishlist.name.clone(),
            games,
            language: wishlist.preference.language.clone(),
            markets: wishlist.preference.markets().into_iter().map(|s| {s.to_string()}).collect()
        }

    }

    //delete /remove
    pub async fn remove_game_from_wishlist(form: web::Json<dto::input::RemoveFromWishlistDTO>, data: web::Data<WishlistController>) -> impl Responder {
        let dto = form.into_inner();
        if let Some(mut wishlist) = data.wishlist_service.get_wishlist(&dto.name).await{
            if wishlist.remove_a_game(&dto.game_id) {
                data.wishlist_service.save(&wishlist).await;
                if let Some(wishlist) = data.wishlist_service.get_wishlist(&dto.name).await{
                    return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(
                        Self::entity_to_dto(wishlist)
                    )
                }else {
                    return HttpResponse::InternalServerError()
                    .json(
                        "could not fetch wishlist "
                    )
                }

            }else {
                return HttpResponse::BadRequest()
                    .body("couldnt remove Item from Wish list")

            }
        }else{
            let error_message = "couldn't get wishlist with id ".to_string() + &dto.name;
            return HttpResponse::BadRequest()
                .body(&error_message)
        }
    }

    
    

    




    pub async fn change_preference(form: web::Json<dto::input::UpdateWishlistPreferenceDTO>, data: web::Data<WishlistController>) -> impl Responder {
        let dto = form.into_inner();
        let wishlist_service = &*data.wishlist_service;
        let game_service = &*data.game_service;
        if let Some(wishlist) = wishlist_service.get_wishlist(&dto.name).await{
            let language: String;
            let preferred_markets: Markets;
            if let Some(dto_language) = dto.language{
                language = dto_language;
            }else{
                language = wishlist.preference.language.clone();
            }
            if let Some(dto_markets) = dto.markets{
                preferred_markets = Markets::from_vec_str(dto_markets).0;
            }else{
                preferred_markets = wishlist.preference.markets_by_default.clone();
            }

            match Self::create_update_game_list(&wishlist, game_service, &language, &preferred_markets, dto.games).await{
                CreateUpdatedGameListResponse::Ok(updated_game_list) =>{
                    let wishlist_pref = crate::core::wishlist::WishlistPreferences{
                        language,
                        markets_by_default: preferred_markets
                    };
                    let wishlist = crate::core::wishlist::Wishlist::new(dto.name.to_string() , wishlist_pref, updated_game_list);
                    wishlist_service.save(&wishlist).await;
                    if let Some(wishlist) = wishlist_service.get_wishlist(&wishlist.name).await{
                        return HttpResponse::Created()
                        .content_type("application/json")
                        .json(
                            Self::entity_to_dto(wishlist)
                        )
                    }else {
                        return HttpResponse::InternalServerError()
                        .json(
                            "could not save wishlist to database"
                        )
                    }
                },
                CreateUpdatedGameListResponse::MissingGame(id)=>{
                    let error_message = "couldn't get game with id ".to_string() + &id;
                    return HttpResponse::InternalServerError()
                        .body(&error_message);
                },
                CreateUpdatedGameListResponse::RedundantGameError(id)=>{
                    let error_message = "game with id ".to_string() + &id +
                        " is redundant int the wishlist";
                    return HttpResponse::InternalServerError()
                        .body(&error_message);
                },
                CreateUpdatedGameListResponse::GameDosentBelongToWishlist(id)=>{
                    let error_message = "The game with the following id does not belong to the wishlist id: ".to_string() + &id;
                    return HttpResponse::InternalServerError()
                        .body(&error_message);

                }
            }

        }else{
            let error_message = "Cannot fetch Wishlist with the provided name".to_string() + &dto.name;
            return HttpResponse::BadRequest()
            .body(&error_message);
        };

    }

    async fn create_update_game_list(wishlist : &Wishlist, game_service: &GameService, language: &str, preferred_markets: &Markets, dto_games_preferred_markets: Option<Vec<dto::input::UpdateWishlistElement>>) -> CreateUpdatedGameListResponse{
        let mut updated_game_list = HashMap::<String, WishlistElement>::new();
        let mut games_preferred_markets = HashMap::<String, Markets>::new();

        for pair in wishlist.games().into_iter(){
            let old_markets = Markets::from_vec_str(pair.1.into_iter().map(|s|{s.to_string()}).collect()).0;
            if wishlist.preference.markets_by_default.equal(&old_markets){
                games_preferred_markets.insert(pair.0.to_string().to_uppercase(), preferred_markets.clone());
            }else{
                games_preferred_markets.insert(pair.0.to_string().to_uppercase(), old_markets);
            }
        }

        if let Some(dto_games_preferred_markets) = dto_games_preferred_markets{
            for wishlist_element in dto_games_preferred_markets.into_iter(){
                if let None = games_preferred_markets.insert(wishlist_element.id.clone().to_uppercase(), Markets::from_vec_str(wishlist_element.markets).0){
                    return CreateUpdatedGameListResponse::GameDosentBelongToWishlist(wishlist_element.id)
                }
            }
        }

        for pair in games_preferred_markets.into_iter(){
            let game = game_service.get_game_info(&pair.0, &language, pair.1.to_vec()).await;
            if let Some(game) = game{
                if let Some(_) = updated_game_list.insert(game.id().to_string(),WishlistElement::new(game,pair.1)){
                    return CreateUpdatedGameListResponse::RedundantGameError(pair.0.clone().to_string());
                }
            }else{
                return CreateUpdatedGameListResponse::MissingGame(pair.0.clone().to_string());
            }
        }

        return CreateUpdatedGameListResponse::Ok(updated_game_list);
    }

    


    fn updated_wishlist_games(){
        
    }

    /* 
    pub fn get_all(data: web::Data<WishlistController>) -> impl Responder{

    }
*/



    pub fn get_web_service(c: web::Data<Self>) -> Scope<> {
        web::scope("/wishlist").
            app_data(c.clone()).
            route("/create", web::post().to(Self::create_wishlist)).
            route("/{name}", web::get().to(Self::get_wishlist)).
            route("/add", web::patch().to(Self::add_game_to_wishlist)).
            route("/update_preference", web::patch().to(Self::change_preference)).
            route("/remove", web::delete().to(Self::remove_game_from_wishlist))


    }

}