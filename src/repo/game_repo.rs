use crate::core::game::Game;
use crate::repo::shared::{MongoEntity, UniqueEntity};
use super::shared;
use crate::core::purchase_option::{PurchaseAvailability};
use mongodb::bson::{doc,Document, Bson};
use mongodb::{Collection, Database};
use std::collections::HashMap;

impl MongoEntity for Game {
    fn to_document(&self) -> Document{
        let vec: Vec<Document> = (&self.purchase_options).into_iter().map(
             |option_by_market| {
                 let options: Vec<Document> = option_by_market.1.iter().map(|option|{
                     option.to_document()
                 }).collect();
                 doc! {
                     "market": option_by_market.0,
                     "options": options,
                 }
             }
         ).collect();

        doc!{
            "id" : &self.id,
            "name" : &self.name,
            "publisher" : &self.publisher,
            "developer" : &self.developer,
            "poster_uri" : &self.poster_uri,
            "store_uri" : &self.store_uri,
            "description" : &self.description,
            "purchase_options" : vec,

        }
    }

    fn create_from_document(doc : &Document) -> Self{
        let id = String::from(doc.get_str("id").unwrap());
        let name = String::from(doc.get_str("name").unwrap());
        let publisher = String::from(doc.get_str("publisher").unwrap());
        let developer = String::from(doc.get_str("developer").unwrap());
        let poster_uri = String::from(doc.get_str("poster_uri").unwrap());
        let store_uri = String::from(doc.get_str("store_uri").unwrap());
        let description = String::from(doc.get_str("description").unwrap());
        let mut purchase_options: HashMap<String, Vec<PurchaseAvailability>> = HashMap::new();
        if let Ok(purchase_options_bson) = doc.get_array("purchase_options"){
            for bson in purchase_options_bson{
                let bson = bson.as_document().unwrap();
                let market = String::from(bson.get_str("market").unwrap());
                let purchase_option = bson.get_array("options").unwrap().iter().map(
                    |option|{
                        let option = option.as_document().unwrap();
                        PurchaseAvailability::create_from_document(option)
                    }
                ).collect();
                purchase_options.insert(market, purchase_option);
            }
        }


        Game{
            id,
            name,
            publisher,
            developer,
            purchase_options,
            poster_uri,
            store_uri,
            description,
        }
    }


}

pub struct  GameRepo{
    data_base_collection : Collection,
    collection_name : String,

}
impl GameRepo{
    pub fn new(data_base : & Database) -> GameRepo{
        let collection_name = String::from("game");
        let data_base_collection = data_base.collection(&collection_name);
        
        GameRepo{
            data_base_collection,
            collection_name
        }
    }

    // pub async fn save(&self, game: &Game){
    //     let option = shared::Repo::get_document_by_id(self, &game.id).await;
    //     if let Some(document) = option{
    //         let res = self.data_base_collection.update_one(doc! {"id": &game.id}, game.to_document(), None).await;
    //         let id = res.unwrap().upserted_id;
    //         if let Some(bson) = id {
    //             if let Bson::ObjectId(id) = bson{
    //                 println!("element id \"{}\" updated into collection \"{}\" with object id \"{}\"",
    //                          &game.id, shared::Repo::get_collection_name(self),id )
    //             }
    //         }
    //         return;
    //     }
    //     let document = game.to_document();
    //     shared::Repo::save_doc(self, document).await;
    // }

}
impl UniqueEntity for Game{
    fn get_unique_selector(&self) -> Document {
        doc! {
            "id": &self.id,
        }
    }
}

impl shared::Repo<Game> for GameRepo{
    fn get_data_base_collection(&self) -> & Collection {
        & self.data_base_collection
    }
    fn get_collection_name(&self) -> & str{
        & self.collection_name
    }

}