/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::sync::Arc;
use super::errors::wishlist::{not_found_by_name, redundant_game, game_not_found, could_not_remove_item};
use actix_web::{Responder, web, HttpResponse, Scope, post, get, patch, delete};
use std::collections::{HashMap};
use utoipa::{OpenApi, openapi};
use crate::game::controller::GameResponse;
use crate::game::GameService;
use crate::wishlist::service::wishlist_service::WishlistService;
use crate::wishlist::wishlist::{Markets, Wishlist, WishlistElement, WishlistPreferences};
use crate::wishlist::controller::dto;
use crate::wishlist::controller::dto::output;

pub struct WishlistController{
    wishlist_service: Arc<WishlistService>,
    game_service: Arc<GameService>
}


pub fn get_web_service(c: web::Data<WishlistController>) -> Scope<> {
    web::scope("/wishlist").
        app_data(c.clone()).
        service(create_wishlist).
        service(get_all).
        service(get_wishlist).
        service(add_game_to_wishlist).
        service(change_preference).
        service(remove_game_from_wishlist).
        service(delete_wishlist)

}

#[derive(OpenApi)]
#[openapi(
paths(
    create_wishlist,
    get_all,
    get_wishlist,
    add_game_to_wishlist,
    change_preference,
    remove_game_from_wishlist,
    delete_wishlist
),
components(
schemas(
    dto::input::CreateWishlist,
    dto::input::WishlistElement,
    dto::input::AddToWishlistDTO,
    dto::input::UpdateWishlistPreferenceDTO,
    dto::input::UpdateWishlistElement,
    dto::input::RemoveFromWishlistDTO
)
),
tags(
()
),
modifiers()
)]
struct WishlistApiDoc;

pub fn get_open_api() -> openapi::OpenApi
{
    WishlistApiDoc::openapi()
}


#[utoipa::path(
context_path = "/wishlist",
request_body = CreateWishlist,
responses(
(status = 200, description = "Create Wishlist", content_type = "application/json" )
)
)]
#[post("/create")]
pub async fn create_wishlist(form: web::Json<dto::input::CreateWishlist>, data: web::Data<WishlistController>) -> impl Responder {
    let dto = form.into_inner();
    let preferred_markets = Markets::from_vec_str(dto.markets).0;
    let wishlist_pref = WishlistPreferences{
        language: dto.language,
        markets_by_default: preferred_markets
    };
    let game_list = data.get_game_list_from_dto(dto.games, &wishlist_pref).await;
    if let Err(error_msg) = game_list
    {
        return HttpResponse::BadRequest()
            .body(error_msg);
    }

    let wishlist = Wishlist::new(dto.name , wishlist_pref, game_list.unwrap());
    data.save_wishlist_properly(wishlist).await
}

#[utoipa::path(
context_path = "/wishlist",
responses(
(status = 200, description = "get all available Wishlists" )
)
)]
#[get("/all")]
pub async fn get_all(data: web::Data<WishlistController>)-> impl Responder{
    let wishlist_service = &*data.wishlist_service;
    let vec: Vec<output::wishlist_info::WishlistInfo> = wishlist_service.get_all().await.into_iter().map(|entity|{
        WishlistController::entity_to_dto(entity)
    }).collect();
    return HttpResponse::Created()
        .content_type("application/json")
        .json(
            vec
        )
}

#[utoipa::path(
context_path = "/wishlist",
responses(
(status = 200, description = "get all wishlist by name" )
)
)]
#[get("/one/{name}")]
pub async fn get_wishlist(name: web::Path<String>, data: web::Data<WishlistController>)-> impl Responder{
    data.fetch_wishlist_properly(&name).await
}

#[utoipa::path(
context_path = "/wishlist",
request_body = AddToWishlistDTO,
responses(
(status = 200, description = "add game to wishlist" )
)
)]
#[patch("/game/add")]
pub async fn add_game_to_wishlist(form: web::Json<dto::input::AddToWishlistDTO>, data: web::Data<WishlistController>)-> impl Responder{
    let dto = form.into_inner();
    let wishlist = data.wishlist_service.get_wishlist(&dto.name).await;
    if let None = wishlist{
        let error_message = format!("couldn't get wishlist with name {}", &dto.name);
        return HttpResponse::BadRequest()
            .body(error_message);
    }

    let mut wishlist = wishlist.unwrap();

    let element = dto.game;
    let markets = element.markets
        .map(|element_markets|Markets::from_vec_str(element_markets).0)
        .unwrap_or(wishlist.preference.markets_by_default.clone());


    let game = data.game_service.get_game_info(&element.id, &wishlist.preference.language, &markets.to_vec()).await;
    if let None = game
    {
        let error_message = format!("couldn't get game with id {}", &element.id);
        return HttpResponse::BadRequest()
            .body(error_message);
    }
    let game = game.unwrap();

    wishlist.add_a_game(game, Some(markets));
    data.save_wishlist_properly(wishlist).await
}


#[utoipa::path(
context_path = "/wishlist",
request_body = UpdateWishlistPreferenceDTO,
responses(
(status = 200, description = "modify the wishlist configuration name, default language, default markets" )
)
)]
#[patch("/update_preference")]
pub async fn change_preference(form: web::Json<dto::input::UpdateWishlistPreferenceDTO>, data: web::Data<WishlistController>) -> impl Responder {
    let dto = form.into_inner();
    let result = data.wishlist_service.get_wishlist(&dto.name).await;
    if let None = result
    {
        return HttpResponse::BadRequest()
            .body(not_found_by_name(&dto.name));
    }
    let wishlist = result.unwrap();

    let wishlist_pref = WishlistPreferences {
        language: dto.language.unwrap_or( wishlist.preference.language),
        markets_by_default: dto.markets.map(|markets| Markets::from_vec_str(markets).0).unwrap_or(wishlist.preference.markets_by_default)
    };
    let wishlist = Wishlist::new(dto.name.to_string(), wishlist_pref, wishlist.games);
    if let None = dto.games
    {
        return data.save_wishlist_properly(wishlist).await;
    }

    let games_to_update = dto.games.unwrap();
    let redundant_games = WishlistController::get_redundant_games(&games_to_update).await;
    if !redundant_games.is_empty()
    {
        let error_message = format!(
            "games with ids {} are redundant, Make sure an element is present only once in the request",
            redundant_games.join(", "));
        return HttpResponse::BadRequest()
            .body(error_message);
    }

    let non_belonging_games = WishlistController::get_not_belonging_games(&wishlist, &games_to_update).await;
    if !non_belonging_games.is_empty()
    {
        let error_message = format!(
            "The games with the following ids <{}> do not belong to the wishlist with name {}",
            non_belonging_games.join(", "),
            &wishlist.name);
        return HttpResponse::BadRequest()
            .body(error_message);
    }

    let wishlist_games = data.create_update_game_list(&wishlist,games_to_update).await;
    if let Err(error_msg) = wishlist_games
    {
        return HttpResponse::InternalServerError()
            .body(error_msg);
    }
    let wishlist_games = wishlist_games.unwrap();
    let wishlist = Wishlist::new(dto.name.to_string(), wishlist.preference, wishlist_games);
    return data.save_wishlist_properly(wishlist).await;

}

#[utoipa::path(
context_path = "/wishlist",
request_body = RemoveFromWishlistDTO,
responses(
(status = 200, description = "modify the wishlist configuration name, default language, default markets" )
)
)]
#[patch("/game/remove")]
pub async fn remove_game_from_wishlist(form: web::Json<dto::input::RemoveFromWishlistDTO>, data: web::Data<WishlistController>) -> impl Responder {
    let dto = form.into_inner();
    let result = data.wishlist_service.get_wishlist(&dto.name).await;
    if let None = result
    {
        return HttpResponse::BadRequest()
            .body(not_found_by_name(&dto.name))
    }
    let mut wishlist = result.unwrap();

    if ! wishlist.remove_a_game(&dto.game_id)
    {
        return HttpResponse::BadRequest()
            .body(could_not_remove_item(&dto.game_id, &dto.name))

    }
    data.save_wishlist_properly(wishlist).await
}

#[utoipa::path(
context_path = "/wishlist",
responses(
(status = 200, description = "delete wish list with name" )
)
)]
#[delete("/{name}")]
pub async fn delete_wishlist(name: web::Path<String>, data: web::Data<WishlistController>)-> impl Responder{
    let wishlist_service = &*data.wishlist_service;
    if let Some(_) = wishlist_service.get_wishlist(&name).await{
        if wishlist_service.delete(&name).await{
            let message = format!("{} is deleted", &name);
            HttpResponse::Ok().body(message)
        }else {
            HttpResponse::InternalServerError().body("Unable to delete wishlist")
        }
    }else{
        let error_message = "Cannot fetch Wishlist with the provided name".to_string() + &name;
        HttpResponse::BadRequest()
            .body(error_message)
    }
}

impl WishlistController{
    pub fn new(wishlist_service: Arc<WishlistService>, game_service: Arc<GameService>) -> Self {
        WishlistController { wishlist_service, game_service }
    }

    pub async fn save_wishlist_properly(&self, wishlist: Wishlist) -> HttpResponse
    {
        self.wishlist_service.save(&wishlist).await;
        self.fetch_wishlist_properly(&wishlist.name).await
    }

    pub async fn fetch_wishlist_properly(&self, wishlist_name: &str) -> HttpResponse
    {
        self.wishlist_service.get_wishlist(wishlist_name).await
            .map(
                |wishlist|HttpResponse::Created()
                                    .content_type("application/json")
                                    .json(Self::entity_to_dto(wishlist))
            )
            .unwrap_or(
                HttpResponse::InternalServerError()
                                    .body(not_found_by_name(&wishlist_name))
            )
    }

    pub async fn get_game_list_from_dto(&self, dto_games: Vec<dto::input::WishlistElement>, wishlist_pref: &WishlistPreferences) -> Result<HashMap<String, WishlistElement>, String>
    {
        let mut game_list = HashMap::<String, WishlistElement>::new();

        for game_dto in dto_games.into_iter(){

            let markets = game_dto.markets
                .map(|markets_list| Markets::from_vec_str(markets_list).0)
                .unwrap_or(wishlist_pref.markets_by_default.clone());

            let game = self.game_service.get_game_info(&game_dto.id, &wishlist_pref.language, &markets.to_vec()).await;
            if let None = game
            {
                return Err(game_not_found(&game_dto.id));
            }
            let game = game.unwrap();
            if let Some(_) = game_list.insert(game_dto.id.clone(),WishlistElement::new(game,markets)){
                return Err(redundant_game(&game_dto.id));
            }
        };
        Ok(game_list)
    }

    async fn get_wishlist_games(&self, vec : Vec<(&str, Vec<&str>)>, language:&str)->Vec<output::wishlist_info::WishlistInfoElement>{
        let mut result = Vec::<output::wishlist_info::WishlistInfoElement>::new();
        for pair in vec.into_iter(){
            let game = self.game_service.get_game_info(pair.0, language, &pair.1.iter().map(|s|{&s[..]}).collect()).await.unwrap();
            let game_info = output::wishlist_info::WishlistInfoElement{
                game: GameResponse::new(game),
                markets: pair.1.iter().map(|s|{s.to_string()}).collect()
            };
            result.push(game_info);
        };
        result
    }

    fn entity_to_dto(wishlist: Wishlist)->output::wishlist_info::WishlistInfo{
        let games: Vec<output::wishlist_info::WishlistInfoElement> = wishlist.games.into_iter().map(|pair|{
            let markets = pair.1.markets().into_iter().map(|str|{str.to_string()}).collect();
            let game = pair.1.game;
            output::wishlist_info::WishlistInfoElement{
                game: GameResponse::new(game),
                markets
            }
        }).collect();
        output::wishlist_info::WishlistInfo{
            name: wishlist.name.clone(),
            games,
            language: wishlist.preference.language.clone(),
            markets: wishlist.preference.markets().into_iter().map(|s| {s.to_string()}).collect()
        }

    }

    async fn get_not_belonging_games(wishlist : &Wishlist, games_to_update: &Vec<dto::input::UpdateWishlistElement>) -> Vec<String>
    {
        games_to_update.iter()
            .filter(|game|!wishlist.games.contains_key(&game.id))
            .map(|game|game.id.clone())
            .collect()
    }

    async fn get_redundant_games(games_to_update: &Vec<dto::input::UpdateWishlistElement>) -> Vec<String>
    {
        let mut encountered_ids = std::collections::HashSet::new();
        let mut redundant_ids = Vec::new();
        for game in games_to_update.into_iter() {
            if !encountered_ids.insert(game.id.to_string()) {
                redundant_ids.push(game.id.to_string());
            };
        };
        redundant_ids
    }

    async fn create_update_game_list(&self, wishlist : &Wishlist, games_to_update: Vec<dto::input::UpdateWishlistElement>) -> Result<HashMap<String, WishlistElement>, String>{
        let mut updated_game_list: HashMap<String, WishlistElement> = Default::default();

        let mut games_preferred_markets = HashMap::<String, Markets>::new();
        for game in wishlist.games()
        {
            games_preferred_markets.insert(game.0.to_string(), Markets::from_vec_str(game.1.iter().map(|str| str.to_string()).collect()).0);
        }
        for game_to_update in games_to_update.into_iter(){
            games_preferred_markets.insert(game_to_update.id.clone().to_uppercase(), Markets::from_vec_str(game_to_update.markets).0);
        }
        for pair in games_preferred_markets.into_iter() {
            let game = self.game_service.get_game_info(&pair.0, &wishlist.preference.language, &pair.1.to_vec()).await;
            if let None = game
            {
                return Err(format!("fetching game with id {} has failed", &pair.0));
            }
            let game = game.unwrap();
            let markets= pair.1;
            updated_game_list.insert(game.id().to_string(),WishlistElement::new(game,markets));
        }
        return Ok(updated_game_list);
    }

}


