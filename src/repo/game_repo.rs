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

        let purchase_options: Vec<Document> = (&self.purchase_options).into_iter().map(
            |purchase_option| {
                let options: Vec<Document> = (&purchase_option.1.purchase_availabilities).into_iter().map(
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

        let descriptions: Vec<Document> = (&self.descriptions).into_iter().map(
            |pair| {
                let desc = pair.1;
                doc! {
                "language": &pair.0,
                "body": doc!{
                    "name" : &pair.1.name,
                    "publisher" : &pair.1.publisher,
                    "developer" : &pair.1.developer,
                    "description" : &pair.1.description,
                    "poster_uri" : &pair.1.poster_uri,
                    },
                }
            }
        ).collect();


        doc!{
            "id" : &self.id,
            "descriptions": descriptions,
            "purchase_options" : purchase_options,
        }
    }


    fn create_from_document(doc : &Document) -> Self{
        let id = String::from(doc.get_str("id").unwrap());
        let mut game_descriptions: HashMap<String, GameDescription> = HashMap::new();

        for description in doc.get_array("descriptions").unwrap().into_iter(){
            if let Bson::Document(desc) = description{
                let body = desc.get_document("body").unwrap();
                game_descriptions.insert(
                    desc.get_str("language").unwrap().to_string(),
                    GameDescription{
                         name: body.get_str("name").unwrap().to_string(),
                         publisher: body.get_str("publisher").unwrap().to_string(),
                         developer: body.get_str("developer").unwrap().to_string(),
                         description: body.get_str("description").unwrap().to_string(),
                         poster_uri: body.get_str("poster_uri").unwrap().to_string()
                     }
                );
            };
        };

        let mut purchase_options: HashMap<String, PurchaseOption> = HashMap::new();
        
        for bson in  doc.get_array("purchase_options").unwrap().into_iter(){
            if let Bson::Document(document) = bson{
                let market = document.get_str("market").unwrap().to_string();
                let store_uri = document.get_str("store_uri").unwrap().to_string();
                let availabilities = document.get_array("availabilities").unwrap().into_iter().map(
                    |doc|{
                        match doc {
                            Bson::Document(doc)=>{
                                PurchaseAvailability::create_from_document(doc)
                            },
                            _ => {
                                panic!()
                            }
                        }
                    }
                ).collect();
                let purchase_option = PurchaseOption{
                    purchase_availabilities: availabilities,
                    store_uri
                };
                purchase_options.insert(market, purchase_option);
            }
        };
        return Self{
            id,
            descriptions: game_descriptions,
            purchase_options
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


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_game_entity_to_document(){

    }
}
