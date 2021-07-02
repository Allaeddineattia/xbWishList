mod client;
mod core;
mod repo;
mod service;

use tokio::task;
use mongodb::{Client, options::ClientOptions};
use std::rc::Rc;
use crate::client::client_service::microsoft_api::{MicrosoftApiService, MARKETS};

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

    let db = Rc::new(client.database("xbWishlist"));

    let purchase_option_service = Rc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
    let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());

    let resp1 = task1.await??;
    let resp2 = task2.await??;
    game_service.get_info_from_response(&resp1, language, market).await?;
    game_service.get_info_from_response(&resp2, language, market).await?;


    Ok(())
}


fn main() {
    //let ids : Vec<String> = env::args().collect();// String::from("9MZ11KT5KLP6"),String::from("9PH339L3Z99C")
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(send_req()){
        Ok(_) => {},
        Err(_) =>{},
    };
    
    //let result = game::send_request(ids).await?;
    //game::get_info_from_response(&result);
    //game::read_from_file();
    //Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn  test_game_get_info()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Rc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Rc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info("9PHKXB8RDKBC", "en-US", vec!["AR", "BR"]).await;
        Ok(())
    }
    #[tokio::test]
    async fn test_game_gHost_runner()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Rc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Rc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info(&"9pdgwzpkcbt6".to_uppercase(), "en-US", vec!["AR", "BR"]).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_game_info()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Rc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Rc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info(&"c3jpd73r365s".to_uppercase(), "en-US", vec!["AR", "BR", "FR", "US"]).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_game_all_markets()-> Result<(), Box<dyn std::error::Error>>{
        let init_db_task = task::spawn(init_db());
        let client = init_db_task.await??;
        let db = Rc::new(client.database("xbWishlist"));
        
        let purchase_option_service = Rc::new(service::purchase_option_service::PurchaseOptionService::new(db.clone()));
        let game_service = service::game_service::GameService::new(db.clone(), purchase_option_service.clone());
        game_service.get_game_info(&"9pdgwzpkcbt6".to_uppercase(), "en-US", MARKETS.keys().copied().collect::<Vec<_>>()).await;
        Ok(())
    }


}
