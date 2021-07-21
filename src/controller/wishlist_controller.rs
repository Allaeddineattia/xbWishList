use crate::service::wishlist_service::WishlistService;
use std::sync::Arc;
use super::dto;
use actix_web::{Responder, web, HttpResponse, Scope};
use std::collections::HashSet;
use std::borrow::Borrow;
use crate::service::game_service::GameService;
use crate::core::wishlist::{Markets, WishlistElement};

pub struct WishlistController{
    wishlist_service: Arc<WishlistService>,
    game_service: Arc<GameService>
}

impl WishlistController{
    pub fn new(wishlist_service: Arc<WishlistService>, game_service: Arc<GameService>) -> Self {
        WishlistController { wishlist_service, game_service }
    }

    // Post /create
    pub async fn create_wishlist(form: web::Json<dto::input::CreateWishlist>, data: web::Data<WishlistController>) -> impl Responder {
        let dto = form.into_inner();
        let mut preferred_markets = Markets::from_vec_str(dto.markets).0;
        let mut game_list = Vec::<WishlistElement>::new();

        for game_dto in dto.games.into_iter(){
            let mut markets;
            if let Some(markets_list) = game_dto.markets{
                markets = Markets::from_vec_str(markets_list).0;
            }else{
                markets = preferred_markets.clone();
            }
            let game = data.game_service.get_game_info(&game_dto.id, &dto.language, markets.to_vec()).await;
            if let Some(game) = game{
                game_list.push(WishlistElement::new(game,markets))
            }else{
                let error_message = "couldn't get game with id ".to_string() + &game_dto.id;
                return HttpResponse::BadRequest()
                    .body(&error_message);
            }
        };

        let wishlist_pref = crate::core::wishlist::WishlistPreferences{
            language: dto.language,
            markets: preferred_markets
        };

        let wishlist = crate::core::wishlist::Wishlist::new(dto.name , wishlist_pref, game_list);
        data.wishlist_service.save(&wishlist).await;
        HttpResponse::Ok()
            .body("created")
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

    pub async fn get_wishlist(web::Path((name)): web::Path<(String)>, data: web::Data<WishlistController>)-> impl Responder{
        let result = data.wishlist_service.get_wishlist(&name).await;
        if let Some(wishlist) = result{

            let result = dto::output::wishlist_info::WishlistInfo{
                name: wishlist.name.clone(),
                games: data.get_wishlist_games(wishlist.games(), &wishlist.preference.language).await,
                language: wishlist.preference.language.clone(),
                markets: wishlist.preference.markets().into_iter().map(|s| {s.to_string()}).collect()
            };
            HttpResponse::Created()
                .content_type("application/json")
                .json(
                    result
                )
        } else{
            HttpResponse::BadRequest().body("Error")
        }

    }

    pub fn get_web_service(c: web::Data<Self>) -> Scope<> {
        web::scope("/wishlist").
            app_data(c.clone()).
            route("/create", web::post().to(Self::create_wishlist)).
            route("/{name}", web::get().to(Self::get_wishlist))

    }

}