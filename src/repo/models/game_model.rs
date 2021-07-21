use std::collections::HashMap;
use crate::core::game::{GameDescription, PurchaseOption, Game};
use crate::repo::shared::{MongoEntity, UniqueEntity};
use mongodb::bson::{doc,Document, Bson};
use crate::core::purchase_option::PurchaseAvailability;

pub struct GameModel{
    pub id: String,
    pub descriptions: HashMap<String, GameDescription>,
    pub purchase_options: HashMap<String, PurchaseOption>,
}

impl GameModel{
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
            if let Some(missing_markets) = missing_markets{
                return FetchGame::MissingMarkets(missing_markets)
            }else {
                FetchGame::Fetched(Game::create(self.id, (language.to_lowercase(), game_description), purchase_options))
            }
        }else{
            FetchGame::MissingDescription(language)

        }

    }

}

pub enum FetchGame<'a>{
    Fetched(Game),
    MissingMarkets(Vec<& 'a str>),
    MissingDescription(& 'a str),
    ElementNotFound(String),
}

impl MongoEntity for GameModel {

    fn to_document(&self) -> Document {
        let purchase_options: Vec<Document> = (self.purchase_options).iter().map(
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

        let descriptions: Vec<Document> = (self.descriptions).iter().map(
            |pair| {
                let desc = pair.1;
                doc! {
                "language": pair.0,
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

    fn from_document(doc: &Document) -> GameModel {
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
                                PurchaseAvailability::from_document(doc)
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
        return GameModel{
            id,
            descriptions: game_descriptions,
            purchase_options
        }
    }
}

impl UniqueEntity for GameModel{
    fn get_unique_selector(&self) -> Document {
        doc! {"id": &self.id}
    }
}