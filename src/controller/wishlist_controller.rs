use crate::service::wishlist_service::WishlistService;
use std::sync::Arc;
use super::dto;
use actix_web::{Responder, web, HttpResponse, Scope};
use std::collections::HashSet;
use std::borrow::Borrow;
use crate::service::game_service::GameService;

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
        let mut preferred_markets = HashSet::<String>::new();
        for market in form.markets.iter(){
            preferred_markets.insert(market.clone());
        }
        let mut game_list = Vec::<(&str, Option<HashSet<&str>>)>::new();
        for game in form.games.iter(){
            if let Some(markets) = &game.markets{
                let mut preferred_markets = HashSet::<(&str)>::new();
                for market in markets.into_iter(){
                    preferred_markets.insert(market);
                }
                game_list.push((&game.id, Some(preferred_markets)));
            }else{
                game_list.push((&game.id, None));
            }
        }

        let wishlist_pref = crate::core::wishlist::WishlistPreferences{
            language: form.language.clone(),
            markets: preferred_markets
        };

        let wishlist = crate::core::wishlist::Wishlist::new(&form.name , wishlist_pref, &game_list);
        data.wishlist_service.save(&wishlist).await;
        HttpResponse::Ok()
            .content_type("application/json")
            .json(
                form.into_inner()
            )
    }

    async fn get_wishlist_games(&self, vec : Vec<(&str, &HashSet<String>)>, language:&str)->Vec<dto::output::wishlist_info::WishlistInfoElement>{
        let mut result = Vec::<dto::output::wishlist_info::WishlistInfoElement>::new();
        for pair in vec.into_iter(){
            let game = self.game_service.get_game_info(pair.0, language, pair.1.iter().map(|s|{&s[..]}).collect()).await.unwrap();
            let game_info = dto::output::wishlist_info::WishlistInfoElement{
                game: dto::output::GameInfo::new(game),
                markets: pair.1.iter().map(|s|{s.clone()}).collect()
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