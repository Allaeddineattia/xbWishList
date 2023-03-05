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



mod client;
mod core;
mod repo;
mod service;
mod controller;

use tokio::task;
use mongodb::{Client, options::ClientOptions};
use crate::client::client_service::microsoft_api::{MicrosoftApiClient, MARKETS};
use actix_web::{get, http, App, Result, HttpResponse, HttpServer, web, error, Error,};
use actix_cors::Cors;

async fn init_db() -> anyhow::Result<mongodb::Client>{
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("XbWishList".to_string());
    Ok(Client::with_options(client_options)?)
}


use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::repo::game_repo::GameRepo;
use crate::repo::purchase_option_repo::PurchaseAvailabilityRepo;
use crate::repo::wishlist_repo::WishlistRepo;
use actix_web::http::header::ContentRangeSpec;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[get("/stream")]
async fn stream(data: web::Data<crate::controller::game_controller::GameController>) -> Result<HttpResponse> {
   Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(
            MyObj{
                name: "hehi".to_string()
            }
        )
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client = init_db().await.unwrap();
    let db = Arc::new(client.database("xbWishlist"));
    let purchase_repo = PurchaseAvailabilityRepo::new();
    let game_repo = Arc::new(GameRepo::new(&db,purchase_repo));
    let wishlist_repo = Arc::new(WishlistRepo::new(&db, game_repo.clone()));
    let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
    let game_service = Arc::new(service::game_service::GameService::new(db.clone(), purchase_option_service.clone(), game_repo.clone()));
    let wishlist_service =  Arc::new(service::wishlist_service::WishlistService::new(game_service.clone(), wishlist_repo));

    let game_controller = web::Data::new(crate::controller::game_controller::GameController::new(game_service.clone()));
    let wishlist_controller = web::Data::new(crate::controller::wishlist_controller::WishlistController::new(wishlist_service, game_service));
    HttpServer::new(move || App::new()
        .wrap(Cors::permissive())
        .service(crate::controller::game_controller::GameController::get_web_service(game_controller.clone()))
        .service(crate::controller::wishlist_controller::WishlistController::get_web_service(wishlist_controller.clone()))
        .service(stream)
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                  error::InternalError::from_response(
                      "",
                          HttpResponse::BadRequest()
                                  .content_type("application/json")
                                  .body(format!(r#"{{"error":"{}"}}"#, err)),
                      )
                      .into()
                  }))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}




//fn main() {
    //let ids : Vec<String> = env::args().collect();// String::from("9MZ11KT5KLP6"),String::from("9PH339L3Z99C")
    //let rt = tokio::runtime::Runtime::new().unwrap();
    //match rt.block_on(send_req()){
    //    Ok(_) => {},
    //    Err(_) =>{},
    //};
    
    //let result = game::send_request(ids).await?;
    //game::get_info_from_response(&result);
    //game::read_from_file();
    //Ok(())
//}



////  -------------------------------- Tests 
/// 
/// 
/// 
///
#[cfg(test)]
mod tests{
    use std::collections::HashSet;

    use super::*;
    use crate::core::wishlist::Markets;

    #[tokio::test]
    async fn  test_game_get_info()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        let purchase_repo = PurchaseAvailabilityRepo::new();
        let game_repo = Arc::new(GameRepo::new(&db,purchase_repo));


        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone(), game_repo.clone());
        game_service.get_game_info("9PHKXB8RDKBC", "en-US", &vec!["AR", "BR"]).await;
        Ok(())
    }
    #[tokio::test]
    async fn test_game_ghost_runner()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        let purchase_repo = PurchaseAvailabilityRepo::new();
        let game_repo = Arc::new(GameRepo::new(&db,purchase_repo));


        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone(), game_repo.clone());
        game_service.get_game_info(&"9pdgwzpkcbt6".to_uppercase(), "en-US", &vec!["AR", "BR"]).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_game_info()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        let purchase_repo = PurchaseAvailabilityRepo::new();
        let game_repo = Arc::new(GameRepo::new(&db,purchase_repo));

        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone(), game_repo.clone());
        game_service.get_game_info(&"c3jpd73r365s".to_uppercase(), "en-US", &vec!["AR", "BR", "FR", "US", "NE"]).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_game_all_markets()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        let purchase_repo = PurchaseAvailabilityRepo::new();
        let game_repo = Arc::new(GameRepo::new(&db,purchase_repo));

        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone(), game_repo.clone());
        game_service.get_game_info("9MZ11KT5KLP6", "en-US", &MARKETS.keys().copied().collect::<Vec<_>>()).await;
        Ok(())
    }
    /*
    #[tokio::test]
    async fn test_wishlist() -> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        let purchase_repo = PurchaseAvailabilityRepo::new();
        let game_repo = Arc::new(GameRepo::new(&db,purchase_repo));

        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = Arc::new(service::game_service::GameService::new(db.clone(), purchase_option_service.clone(),game_repo.clone()));
        let wishlist_service = service::wishlist_service::WishlistService::new(game_service.clone(), &*db);
        let mut preferred_markets = Markets::new();
        preferred_markets.add_market("BR".to_string());
        preferred_markets.add_market("AR".to_string());
        preferred_markets.add_market("US".to_string());
        preferred_markets.add_market("FR".to_string());

        let mut game_list = Vec::<(&str, Option<HashSet<&str>>)>::new();
        game_list.push(("9MZ11KT5KLP6", None));
        game_list.push( ( "9nxvc0482qs5", Some(["BR", "GR"].iter().cloned().collect()) ) );

        let wishlist_pref = crate::core::wishlist::WishlistPreferences{
            language: "en-US".to_string(),
            markets: preferred_markets
        };

        let wishlist = crate::core::wishlist::Wishlist::new("4778".to_string(), wishlist_pref, &game_list);
        wishlist_service.save(&wishlist).await;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_wishlist() -> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));

        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = Arc::new(service::game_service::GameService::new(db.clone(), purchase_option_service.clone()));
        let wishlist_service = service::wishlist_service::WishlistService::new(game_service.clone(), &*db);

        if let Some(wishlist) = wishlist_service.get_wishlist("4778").await{
            wishlist_service.print_wishlist(&wishlist).await;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));

        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = Arc::new(service::game_service::GameService::new(db.clone(), purchase_option_service.clone()));
        game_service.search_game("Devil may", "US").await;

        Ok(())
    }
*/
}
