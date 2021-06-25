use crate::core::game::{Game, PurchaseOption, GameDescription};
use crate::repo::shared::{MongoEntity, UniqueEntity};
use super::shared;
use crate::core::purchase_option::PurchaseAvailability;
use mongodb::bson::{doc,Document, Bson};
use mongodb::{Collection, Database};
use std::collections::HashMap;
use std::any::Any;


struct GameEntity{
    pub id: String,
    pub descriptions: HashMap<String, GameDescription>,
    pub purchase_options: HashMap<String, PurchaseOption>,
}
impl GameEntity{
    pub fn add_game(&mut self, game: &Game) -> Result<(), &'static str> {
        if &self.id != game.id() {
            return Err("not compatible ");
        };
        let g = (*game.description()).clone();
        self.descriptions.insert(game.description_language().to_string(), g);
        for option in game.purchase_options(){
            let o = (*option.1).clone();
            self.purchase_options.insert(option.0.to_string(), o);
        }
        Ok(())
    }
}



impl MongoEntity for GameEntity {

    fn to_document(&self) -> Document{

        let purchase_options: Vec<Document> = (self.purchase_options).into_iter().map(
            |purchase_option| {
                let options: Vec<Document> = purchase_option.1.purchase_availabilities.into_iter().map(
                    |availability|{
                        availability.to_document()
                    }
                ).collect();
                doc! {
                    "market": purchase_option.0,
                    "store_uri": &purchase_option.1.store_uri,
                    "availabilities": options,
                }
            }
        ).collect();
        let descriptions = (self.des).into_iter().map(
            |purchase_option| {
                let options: Vec<Document> = purchase_option.1.purchase_availabilities.into_iter().map(
                    |availability|{
                        availability.to_document()
                    }
                ).collect();
                doc! {
                    "market": purchase_option.0,
                    "store_uri": &purchase_option.1.store_uri,
                    "availabilities": options,
                }
            }
        ).collect();
        doc! {
            "language": self.description_language(),
            "body": doc!{
                "name" : self.name(),
                "publisher" : self.publisher(),
                "developer" : self.developer(),
                "description" : self.description(),
                "poster_uri" : self.poster_uri(),
            },
        };

        doc!{
            "id" : self.id(),
            "description": description,
            "purchase_options" : vec,

        }
    }


    fn create_from_document(doc : &Document) -> Self{
        let id = String::from(doc.get_str("id").unwrap());
        if let Bson::Document(description) = doc.get_array("descriptions").unwrap().get(0).unwrap(){
            let name = String::from(description.get_str("name").unwrap());
            let publisher = String::from(description.get_str("publisher").unwrap());
            let poster_uri = String::from(description.get_str("poster_uri").unwrap());
            let developer = String::from(description.get_str("developer").unwrap());
            let poster_uri =  String::from(description.get_str("poster_uri").unwrap());
        }

        let store_uri = String::from(doc.get_str("").unwrap());

        let purchase_options: Vec<PurchaseAvailability> = doc.get_array("purchase_options").unwrap()
                            .into_iter().map(|bson| {
                                if let Bson::Document(document) = bson{
                                    PurchaseAvailability::create_from_document(document)
                                }else{
                                    panic!();
                                }
                            }).collect();
        

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

impl UniqueEntity for GameEntity{
    fn get_unique_selector(&self) -> Document {
        doc! {"id": &self.id}
    }
}

impl shared::Repo<GameEntity> for GameRepo{
    fn get_data_base_collection(&self) -> & Collection {
        & self.data_base_collection
    }
    fn get_collection_name(&self) -> & str{
        & self.collection_name
    }


}