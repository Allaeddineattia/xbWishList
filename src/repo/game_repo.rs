use crate::core::game::Game;
use crate::repo::shared::MongoEntity;
use super::shared;
use crate::core::purchase_option::PurchaseAvailibility;
use mongodb::bson::{doc,Document, Bson};
use mongodb::{Collection, Database};

impl MongoEntity for Game {
    fn to_document(&self) -> Document{
        let vec: Vec<Document> = (&self.purchase_options).into_iter().map(
            |option| {
                option.1.to_document()
            }
        
        ).collect();
        let discription = doc!{
            "id" : self.id(),
            "name" : self.name(),
            "publisher" : self.publisher(),
            "poster_uri" : self.poster_uri(),


        };
        doc!{
            "id" : self.id(),
            "name" : self.name(),
            "publisher" : self.publisher(),
            "poster_uri" : self.poster_uri(),
            "store_uri" : self.store_uri(),
            "purchase_options" : vec,

        }
    }
    fn create_from_document(doc : &Document) -> Self{
        let id = String::from(doc.get_str("id").unwrap());
        let name = String::from(doc.get_str("name").unwrap());
        let publisher = String::from(doc.get_str("publisher").unwrap());
        let poster_uri = String::from(doc.get_str("poster_uri").unwrap());
        let store_uri = String::from(doc.get_str("store_uri").unwrap());
        let purchase_options: Vec<PurchaseAvailibility> = doc.get_array("purchase_options").unwrap()
                            .into_iter().map(|bson| {
                                if let Bson::Document(document) = bson{
                                    PurchaseAvailibility::create_from_document(document)
                                }else{
                                    panic!();
                                }
                            }).collect();
        Game{
            id,
            name,
            publisher,
            purchase_options,
            poster_uri,
            store_uri,
        }
    }
}

pub struct  GameRepo{
    data_base_collection : Collection,
    collection_name : String ,

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
}

impl shared::Repo for GameRepo{
    fn get_data_base_collection(&self) -> & Collection {
        & self.data_base_collection
    }
    fn get_collection_name(&self) -> & str{
        & self.collection_name
    }
}