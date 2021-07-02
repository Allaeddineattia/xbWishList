use crate::core::game::{Game, PurchaseOption, GameDescription};
use crate::repo::shared::{MongoEntity, Repo, UniqueEntity};
use super::shared;
use crate::core::purchase_option::PurchaseAvailability;
use mongodb::bson::{doc,Document, Bson};
use mongodb::{Collection, Database};
use std::collections::HashMap;
use std::any::Any;



pub struct GameEntity{
    pub id: String,
    pub descriptions: HashMap<String, GameDescription>,
    pub purchase_options: HashMap<String, PurchaseOption>,
}

pub enum FetchGame<'a>{
    Fetched(Game),
    MissingMarkets(Vec<& 'a str>),
    MissingDescription(& 'a str),
    ElementNotFound(String),
}

impl GameEntity{
    pub fn add_info(&mut self, game: &Game) -> Result<(), &'static str> {
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

    pub fn new(id: &str) -> Self{
        Self{
            id: id.to_string(),
            descriptions: HashMap::<String, GameDescription>::new(),
            purchase_options: HashMap::<String, PurchaseOption>::new()
        }
    }

    pub fn print (&self){
        println!("hehi");
    }



    pub fn get_game<'a>(mut self, language: & 'a str, markets: &Vec<& 'a str>)-> FetchGame<'a>{
        let game_description = self.descriptions.remove(language);

        if let Some(game_description) = game_description {
            let mut purchase_options = HashMap::<String, PurchaseOption>::new();
            let mut missing_markets: Option<Vec<& 'a str>> = None;
            for market in markets{
                if let Some(purchase_option) = self.purchase_options.remove(*market){
                    purchase_options.insert(market.to_string(), purchase_option);
                    
                }else{
                    if let Some(missing_markets) = &mut missing_markets{
                        missing_markets.push(market);
                    }else {
                        missing_markets = Some(vec![market]);
                    }
                    println!("purchase options for market {} not found", *market);
                }
                
            };
            FetchGame::Fetched(Game::create(self.id, (language.to_lowercase(), game_description), purchase_options))

            
        }else{
            FetchGame::MissingDescription(language)
            
        }

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

    pub async fn fetch_game<'a>(&self, id:&str , language: & 'a str, markets: &Vec<& 'a str>)-> FetchGame<'a>{
        let query = doc! {
            "id": id,
            "descriptions.language":language,
            "purchase_options.market": doc!{
                "$in": markets,
            }

        };
        let game_entity = self.fetch_by_query(query).await;
        if let Some(game_entity) = game_entity  {
            return  game_entity.get_game(language, markets);
        }else{
            FetchGame::ElementNotFound("hehi".to_string())
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
