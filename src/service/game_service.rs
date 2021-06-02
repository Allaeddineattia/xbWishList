use crate::client::input_dto::catalog_response;
use crate::core::game;
use crate::repo::shared::MongoEntity;
use crate::repo::shared::Repo;
use mongodb::Database;
use crate::repo::game_repo::GameRepo;
use std::rc::Rc;
use crate::client::client_service::microsoft_api::XboxLiveLanguage;
use crate::core::purchase_option::{PurchaseAvailability, PurchaseOption};

pub struct GameService {
    db : Rc<Database>,
}

impl GameService{

    pub fn new(db : Rc<Database>) -> Self{
        GameService{db}
    }

    fn get_sales(product: &catalog_response::Product, language: &XboxLiveLanguage) -> PurchaseOption{
        let mut sales: Vec<PurchaseAvailability> = vec![];
        for sku_availability in product.display_sku_availabilities.iter()
        {
            for availability in sku_availability.availabilities.iter()
            {
                if availability.actions.iter().find(|&x| x == "Purchase") != None
                {
                    sales.push(PurchaseAvailability::new(availability));
                }
            }
        }
        sales
    }

    pub fn abstract_result_to_game(&self, result: &catalog_response::Response, language: &XboxLiveLanguage){

        for product in result.products.iter() {
            let mut name = String::from("null");
            let mut developer_name = String::from("null");
            let mut publisher_name = String::from("null");
            let mut poster_uri = String::from("null");
            let id = product.product_id.clone();
            for localized_properties in product.localized_properties.iter(){
                name = localized_properties.product_title.clone();
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
            let store_uri = String::from("https://www.microsoft.com/") + &language.local() + "/p/" +
                &name.trim().replace(" ", "-").replace(":", "")
                    .replace("|", "").replace("&", "").to_lowercase() + "/"
                + &product.product_id;

            GameService::get_sales(product);

        }


    }

    pub async fn get_info_from_response( &self, result: &catalog_response::Response) -> anyhow::Result<()>
    {
        // for product in result.products.iter(){
        //     let result: game::Game = game::Game::new(product);
        //     let id = result.id.clone();
        //     let game_repo = GameRepo::new(&*self.db);
        //     let doc = result.to_document();
        //     game_repo.save_doc(doc).await;
        //     let result = game_repo.get_document_by_id(&id).await;
        //     println!("id {}", id);
        //     let result = result.unwrap();
        //     let result = game::Game::create_from_document(&result);
        //     result.print();
        // }
        Ok(())
    }
}


