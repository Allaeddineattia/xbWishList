use crate::client::input_dto::catalog_response;
use crate::client::input_dto;
use crate::core::game;
use crate::repo::shared::MongoEntity;
use crate::repo::shared::Repo;
use mongodb::Database;
use crate::repo::game_repo::GameRepo;
use std::rc::Rc;
use crate::client::client_service::microsoft_api::XboxLiveLanguage;
use crate::core::purchase_option::{PurchaseAvailability};
use crate::service::purchase_option_service::PurchaseOptionService;
use crate::core::game::Game;
use crate::core::game::Property;

pub struct GameService {
    db : Rc<Database>,
    purchase_option_service: Rc<PurchaseOptionService>,
    game_repo: GameRepo,
}

impl GameService{

    pub fn new(db: Rc<Database>, purchase_option_service: Rc<PurchaseOptionService>) -> Self {
        GameService { db: db.clone(), purchase_option_service, game_repo:GameRepo::new(&*db) }
    }

    fn get_properties(&self, properties : & input_dto::product_property::ProductProperties) -> Vec<Property>{
        let mut result = Vec::<Property>::new();
        if let Some(attributes) = properties.attributes{
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
                };
            }
        };
        result
        
    }

    fn abstract_product_to_game(&self, product: &catalog_response::Product, language: & str, market: & str) -> Game{
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

        let store_uri = String::from("https://www.microsoft.com/") + language + "/p/" +
            &name.trim().replace(" ", "-").replace(":", "")
                .replace("|", "").replace("&", "").to_lowercase() + "/"
            + &product.product_id;

        let mut game = Game::new(id, name, publisher_name, developer_name,
                                 poster_uri, store_uri, description, language.to_string(), self.get_properties(&product.properties.unwrap()));
        let sales = self.purchase_option_service.get_sales(product);
        game.add_purchase_option(market, store_uri, sales);
        game
    }

    pub fn abstract_result_to_games(&self, result: &catalog_response::Response, language: & str, market: & str) -> Vec<Game>{
        result.products.iter().map(|product|{
            self.abstract_product_to_game(product, language, market)
        }).collect()
    }


    pub async fn get_info_from_response( &self, result: &catalog_response::Response, language: & str, market: & str) -> anyhow::Result<()>
    {
        for product in result.products.iter(){
            let result: game::Game = self.abstract_product_to_game(product, language, market);
            //self.game_repo.save(&result).await;
            //let result = self.game_repo.fetch(&result).await;
            //let result = result.unwrap();
            result.print();
        }
        Ok(())
    }



}


