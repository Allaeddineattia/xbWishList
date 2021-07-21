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

use crate::client::input_dto::catalog_response;
use crate::client::input_dto;
use crate::core::game;
use crate::repo::shared::MongoEntity;
use crate::repo::shared::Repo;
use mongodb::Database;
use crate::repo::game_repo::{GameRepo, GameEntity};
use std::collections::HashMap;
use std::future::Future;
use std::rc::Rc;
use crate::client::client_service::microsoft_api::{XboxLiveLanguage, MicrosoftApiService, MARKETS};
use crate::core::purchase_option::{PurchaseAvailability};
use crate::service::purchase_option_service::PurchaseOptionService;
use crate::core::game::Game;
use crate::core::game::Property;
use tokio::task;
use std::sync::Arc;
use crate::client::input_dto::search_response::{SearchItem, SearchItemProduct};
use crate::client::input_dto::catalog_response::Response;
use crate::repo::models::game_model::{FetchGame, GameModel};

pub struct GameService {
    db : Arc<Database>,
    purchase_option_service: Arc<PurchaseOptionService>,
    game_repo: Arc<GameRepo>,
}

impl GameService{

    pub fn new(db: Arc<Database>, purchase_option_service: Arc<PurchaseOptionService>, game_repo: Arc<GameRepo>) -> Self {
        GameService { 
            db: db.clone(), 
            purchase_option_service, 
            game_repo,
        }
    }

    fn get_properties(&self, properties : & input_dto::product_property::ProductProperties) -> Vec<Property>{
        let mut result = Vec::<Property>::new();
        if let Some(attributes) = &properties.attributes{
            for attribute in attributes.iter(){
                match &attribute.name[..] {
                    "CapabilityXboxEnhanced" => {
                        result.push(Property::XboxOneXEnhanced);
                    },
                    "Capability4k" => {
                        result.push(Property::UltraHD4K);
                    },
                    "XboxLive" => {
                        result.push(Property::XboxLive);
                    },
                    "CapabilityHDR" => {
                        result.push(Property::HDR);
                    },
                    "XPA" => {
                        result.push(Property::XboxPlayAnywhere);
                    },
                    "SharedSplitScreen" => {
                        result.push(Property::SharedSplitScreen);
                    },
                    "CrossPlatformMultiPlayer" => {
                        result.push(Property::CrossPlatformMultiPlayer);
                    },
                    "CrossPlatformCoOp" => {
                        result.push(Property::CrossPlatformCoOp);
                    },
                    "VREnabled" => {
                        result.push(Property::WindowsMixedReality);
                    },
                    "RayTracing" => {
                        result.push(Property::RayTracing);
                    },
                    "60fps" => {
                        result.push(Property::FPS60);
                    },
                    "120fps" => {
                        result.push(Property::FPS120);
                    },
                    "ConsoleGen9Optimized" => {
                        result.push(Property::OptimizedForSeriesXAndS);
                    },
                    "GameStreaming" => {
                        result.push(Property::CloudEnabled);
                    },
                    "ConsoleCrossGen" => {
                        result.push(Property::SmartDelivery);
                    },
                    "ConsoleKeyboardMouse" => {
                        result.push(Property::ConsoleKeyboardMouse);
                    },
                    "PcGamePad" => {
                        result.push(Property::PcGamePad);
                    },
                    "XboxLiveCrossGenMP" => {
                        result.push(Property::CrossGenMultiPlayer);
                    },
                    "XblOnlineMultiPlayer" => {
                        let min = attribute.minimum.unwrap() as u16;
                        let max = attribute.minimum.unwrap() as u16;
                        result.push(Property::OnlineMultiplayer(min, max));
                    },
                    "XblLocalMultiPlayer" => {
                        let min = attribute.minimum.unwrap() as u16;
                        let max = attribute.minimum.unwrap() as u16;
                        result.push(Property::LocalMultiplayer(min, max));
                    },
                    "XblLocalCoop" => {
                        let min = attribute.minimum.unwrap() as u16;
                        let max = attribute.minimum.unwrap() as u16;
                        result.push(Property::LocalCoop(min, max));
                    },
                    "XblOnlineCoop" => {
                        let min = attribute.minimum.unwrap() as u16;
                        let max = attribute.minimum.unwrap() as u16;
                        result.push(Property::OnlineCoop(min, max));
                    },
                    _ => {}
                };
            }
        };
        result
        
    }

    fn abstract_product_to_game(&self, product: &catalog_response::Product, language: & str, market: &  XboxLiveLanguage::<'static>) -> Game{
        let mut name = String::from("null");
        let mut developer_name = String::from("null");
        let mut publisher_name = String::from("null");
        let mut poster_uri = String::from("null");
        let mut description = String::from("null");
        let id = product.product_id.clone();
        for localized_properties in product.localized_properties.iter(){
            name = localized_properties.product_title.clone();
            if let Some(desc) = &localized_properties.product_description{
                description = desc.clone();
            }
            if let Some(develop_name) = &localized_properties.developer_name {
                developer_name = develop_name.clone();
            }
            if let Some(publisher) = &localized_properties.publisher_name {
                publisher_name = publisher.clone();
            }
            for image in localized_properties.images.iter() {
                if image.image_purpose == "Poster" {
                    let uri = String::from("http:") + &image.uri;
                    poster_uri = uri;
                }
            }
        }

        let store_uri = String::from("https://www.microsoft.com/") + market.local() + "/p/" +
            &name.trim().replace(" ", "-").replace(":", "").replace("'", "")
                .replace("|", "").replace("&", "").to_lowercase() + "/"
            + &product.product_id;

        let properties = self.get_properties(&product.properties.as_ref().unwrap());

        let mut game = Game::new(id, name, publisher_name, developer_name,
                                 poster_uri,  description, language.to_string(), properties);
        let sales = self.purchase_option_service.get_sales(product);
        game.add_purchase_option(market.short_id(), store_uri, sales);
        game
    }

    pub fn abstract_result_to_games(&self, result: &catalog_response::Response, language: & str, market: &  XboxLiveLanguage::<'static>) -> Vec<Game>{
        result.products.iter().map(|product|{
            self.abstract_product_to_game(product, language, market)
        }).collect()
    }

    pub async fn save_game(&self, game:&Game){
        let mut game_entity: GameModel;
        let fetch_result = self.game_repo.fetch_by_id(game.id()).await;
        if let Some(entity) = fetch_result{
            game_entity = entity;
        }else{
            game_entity = GameModel::new(game.id());
        }
        game_entity.add_info(game);
        self.game_repo.save(&game_entity).await;
    }

    pub async fn save_response(&self, result: &catalog_response::Response, language: & str, market: &  XboxLiveLanguage::<'static>) -> anyhow::Result<()>
    {
        for product in result.products.iter(){
            let result: game::Game = self.abstract_product_to_game(product, language, market);
            self.save_game(&result).await;
        }
        Ok(())
    }

    async fn cure_description_missing (&self, id: &str, language: &str){
        println!("game description is missing trying to cure the problem");
        let market = & crate::client::client_service::microsoft_api::UNITED_STATES;
        let result = MicrosoftApiService::get_games(vec![id.to_string()], language, market.short_id()).await;
        if let Ok(result) = result{
            self.save_response(&result, language, &market).await;
        }

    }

    async fn cure_markets_missing(&self, id: &str, markets: &Vec<&str>){
        println!("game description is missing markets to cure the problem");
        let mut tasks : Vec<(&XboxLiveLanguage,task::JoinHandle<Result<crate::client::input_dto::catalog_response::Response, anyhow::Error>>)> = vec![];
        for market in markets{
            let market = MARKETS.get(market);
            if let Some(market) = market{
                let task = task::spawn(MicrosoftApiService::get_games(vec![id.to_string()], market.local(), market.short_id()));
                tasks.push( (market,task));
            } 

        }

        for task_to_join in tasks{
            let result = task_to_join.1.await.unwrap().unwrap();
            self.save_response(&result, task_to_join.0.local(), task_to_join.0).await;
            
        }

    }

    pub async fn get_game_info(&self, id:&str , language: & str, markets: Vec<& str>) -> Option<Game>{
        let id = &id.to_uppercase()[..];
        let mut description_is_cured = false;
        let mut missing_markets_are_cured = false;
        let mut not_found = false;
        while !description_is_cured || !missing_markets_are_cured || !not_found{
            let game = self.game_repo.fetch_game(id, language, &markets).await;
            match game{
                FetchGame::ElementNotFound(error_message)=>{
                    println!("element is missing {} ", error_message);
                    if description_is_cured{
                        println!("no game found with this id {}", id);
                        not_found = true;
                    }

                    if !description_is_cured{
                        self.cure_description_missing(id, language).await;
                    }
                    description_is_cured = true;
                    if !missing_markets_are_cured{
                        self.cure_markets_missing(id, &markets).await;
                    }
                    missing_markets_are_cured = true;
                    
                },
                FetchGame::Fetched(game)=>{
                    println!("\n-----------------\n");
                    game.print_price();
                    return Some(game);
                }
                FetchGame::MissingDescription(language)=>{
                    if description_is_cured{
                        println!("couldn't find a discription of game with id {} for language {}", id, language);
                        return None;
                    };
                    self.cure_description_missing(id, language).await;
                    description_is_cured = true;
                }
                FetchGame::MissingMarkets(missing_markets)=>{
                    if missing_markets_are_cured{
                        println!("the game with id {} is not supported in the following markets {:#?}", id, missing_markets);
                        return None;
                    };
                    self.cure_markets_missing(id, &missing_markets).await;
                    missing_markets_are_cured = true;
                }
            }
        };
        None
        
    }

    pub async fn get_game_info_from_all_markets(&self, id: &str){
        let mut tasks : Vec<(&XboxLiveLanguage,task::JoinHandle<Result<crate::client::input_dto::catalog_response::Response, anyhow::Error>>)> = vec![];
        for market in MARKETS.into_iter(){
            let market = market.1;
            let task = task::spawn(MicrosoftApiService::get_games(vec![id.to_string()], market.local(), market.short_id()));
                tasks.push( (market,task));
        }
        for task_to_join in tasks{
            let result = task_to_join.1.await.unwrap().unwrap();
            self.save_response(&result, task_to_join.0.local(), task_to_join.0).await;
            
        }
    }

    pub async fn search_game(&self, query: &str, language: &str) -> Vec<SearchItemProduct> {
        let market = MARKETS.get(language);
        let mut vec = Vec::<SearchItemProduct>::new();
        if let Some(market) = market{
            let result = MicrosoftApiService::search_games(query, &market.local(), &market.short_id()).await;
            if let Ok(search_response) = result{
                for item in search_response.results.into_iter(){
                    for mut product in item.products.into_iter(){
                        product.icon = "https:".to_string() + &product.icon;
                        println!("product found \nid: {} \ntitle: {} \nimage url: {}", product.product_id, product.title, product.icon);
                        vec.push(product);
                    }
                }
            }
        } else {
            println!("language {} not supported", language);
        }
        vec
        
    }

}


trait GameAPI{

}





