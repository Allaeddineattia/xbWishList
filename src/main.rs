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
use crate::client::client_service::microsoft_api::{MicrosoftApiService, MARKETS};
use actix_web::{get, App, Result, HttpResponse, HttpServer, web};

async fn init_db() -> anyhow::Result<mongodb::Client>{
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("XbWishList".to_string());
    Ok(Client::with_options(client_options)?)
}


async fn send_req() -> Result<(), Box<dyn std::error::Error>>{
    let init_db_task = task::spawn(init_db());
    let language = client::client_service::microsoft_api::UNITED_STATES.local();
    let market = &client::client_service::microsoft_api::ARGENTINA;
    let task1 = task::spawn(
        MicrosoftApiService::get_games(vec!["9nn50lxzt18z".to_string(), "9phkxb8rdkbc".to_string()],
                                       language,market.short_id() ));

    let task2 = task::spawn(
        MicrosoftApiService::get_games(vec!["9n2zdn7nwqkv".to_string(), "9ph339l3z99c".to_string()],
                                        language, market.short_id()));// nier: bppzvt8bz15n //9PH339L3Z99C / fifa 9nn50lxzt18z / starwars c2csdtscbz0c
    let client = init_db_task.await??;

    let db = Arc::new(client.database("xbWishlist"));

    let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
    let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());

    let resp1 = task1.await??;
    let resp2 = task2.await??;
    game_service.get_info_from_response(&resp1, language, market).await?;
    game_service.get_info_from_response(&resp2, language, market).await?;


    Ok(())
}

use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

    let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
    let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());

    let c = web::Data::new(crate::controller::game_controller::GameController::new(game_service));
    HttpServer::new(move || App::new().service(crate::controller::game_controller::GameController::get_web_service(c.clone()))
        .service(stream)
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


#[cfg(test)]
mod tests{
    use std::collections::HashSet;

    use super::*;

    #[tokio::test]
    async fn  test_game_get_info()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info("9PHKXB8RDKBC", "en-US", vec!["AR", "BR"]).await;
        Ok(())
    }
    #[tokio::test]
    async fn test_game_ghost_runner()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info(&"9pdgwzpkcbt6".to_uppercase(), "en-US", vec!["AR", "BR"]).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_game_info()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info(&"c3jpd73r365s".to_uppercase(), "en-US", vec!["AR", "BR", "FR", "US", "NE"]).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_game_all_markets()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info("9MZ11KT5KLP6", "en-US", MARKETS.keys().copied().collect::<Vec<_>>()).await;
        Ok(())
    }
    #[tokio::test]
    async fn test_wishlist() -> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Arc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Arc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = Arc::new(service::game_service::GameService::new(db.clone(), purchase_option_service.clone()));
        let wishlist_service = service::wishlist_service::WishlistService::new(game_service.clone(), &*db);
        let mut prefered_markets = HashSet::new();
        prefered_markets.insert("BR".to_string());
        prefered_markets.insert("AR".to_string());
        prefered_markets.insert("US".to_string());
        prefered_markets.insert("FR".to_string());

        let mut game_list = Vec::<(&str, Option<HashSet<&str>>)>::new();
        game_list.push(("9MZ11KT5KLP6", None));
        game_list.push( ( "9nxvc0482qs5", Some(["BR", "GR"].iter().cloned().collect()) ) );

        let wishlist_pref = crate::core::wishlist::WishlistPreferences{
            language: "en-US".to_string(),
            markets: prefered_markets
        };

        let wishlist = crate::core::wishlist::Wishlist::new("4778", wishlist_pref, &game_list);
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

}
