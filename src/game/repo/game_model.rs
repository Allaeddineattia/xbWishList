use std::collections::HashMap;
use std::ops::Add;
use chrono::{DateTime, Duration, Timelike, TimeZone, Utc};
use mongodb::bson;
use mongodb::bson::{doc, Document, Bson};
use crate::game::game::{Game, GameDescription};
use crate::game::purchase_option::{PurchaseAvailability, PurchaseOption};
use crate::shared::repository::{MongoEntity, UniqueEntity};

pub struct GameModel{
    pub id: String,
    pub descriptions: HashMap<String, GameDescription>,
    pub purchase_options: HashMap<String, PurchaseOption>,
    pub expire_at: DateTime<Utc>,
}

impl MongoEntity for GameModel {

    fn to_document(&self) -> Document {
        let purchase_options: Vec<Document> = self.get_purchase_options();

        let descriptions: Vec<Document> = self.get_descriptions();

        doc!{
            "id" : &self.id,
            "descriptions": descriptions,
            "purchase_options" : purchase_options,
            "expire_at" : mongodb::bson::DateTime::from_chrono( self.expire_at)
        }
    }

    fn from_document(doc: &Document) -> GameModel {
        let id = String::from(doc.get_str("id").unwrap());
        let descriptions = Self::descriptions_from_document_array(doc.get_array("descriptions").unwrap());
        let purchase_options = Self::purchase_options_from_document(doc.get_array("purchase_options").unwrap());
        let expire_at: DateTime<Utc> = doc.get_datetime("expire_at").unwrap().clone().into();

        return GameModel{
            id,
            descriptions,
            purchase_options,
            expire_at
        }
    }
}

impl GameModel{
    pub fn add_info(&mut self, game: &Game) -> Result<(), &'static str> {
        if &self.id != game.id() {
            return Err("not compatible ");
        };
        self.descriptions.insert(game.description_language().to_string(), game.description().clone());
        for option in game.purchase_options(){
            let o = (*option.1).clone();
            self.purchase_options.insert(option.0.to_string(), o);
        }
        Ok(())
    }

    fn get_expire_at() -> DateTime<Utc>
    {
        let time = Utc::now().naive_utc().date().and_hms_opt(8, 0, 0).unwrap();
        if Utc::now().hour() < 8
        {
            Utc.from_utc_datetime(&time)
        } else
        {
            Utc.from_utc_datetime(&time).add(Duration::days(1))
        }
    }

    pub fn new(id: &str) -> Self{
        Self{
            id: id.to_string(),
            descriptions: HashMap::<String, GameDescription>::new(),
            purchase_options: HashMap::<String, PurchaseOption>::new(),
            expire_at: Self::get_expire_at(),
        }
    }



    fn get_purchase_options(&self) -> Vec<Document>
    {
        self.purchase_options.iter().map(
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
        ).collect()
    }

    fn get_descriptions(&self) -> Vec<Document>
    {
        self.descriptions.iter().map(
            |pair| {
                let desc = pair.1;
                doc! {
                "language": pair.0,
                "body": doc!{
                    "name" : &desc.name,
                    "publisher" : &desc.publisher,
                    "developer" : &desc.developer,
                    "description" : &desc.description,
                    "poster_uri" : &desc.poster_uri,
                    },
                }
            }
        ).collect()
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
                return FetchGame::MissingMarkets(self.id.clone(),missing_markets)
            }else {
                FetchGame::Fetched(Game::create(self.id, (language.to_lowercase(), game_description), purchase_options))
            }
        }else{
            FetchGame::MissingDescription(self.id.clone(),language)

        }

    }

    fn descriptions_from_document_array(description_array: &bson::Array)->HashMap<String, GameDescription>
    {
        let mut game_descriptions: HashMap<String, GameDescription> = HashMap::new();

        for bson in description_array.into_iter(){
            if let Bson::Document(desc) = bson{
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
        game_descriptions
    }

    fn purchase_options_from_document(purchase_options_array: &bson::Array) -> HashMap<String, PurchaseOption>
    {
        let mut purchase_options: HashMap<String, PurchaseOption> = HashMap::new();

        for bson in  purchase_options_array.into_iter(){
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
        purchase_options
    }

}

pub enum FetchGame<'a>{
    Fetched(Game),
    MissingMarkets(String,Vec<& 'a str>),
    MissingDescription(String,& 'a str),
    ElementNotFound(String),
}



impl UniqueEntity for GameModel{
    fn get_unique_selector(&self) -> Document {
        doc! {"id": &self.id}
    }
}

////  -------------------------------- Tests
///
///
///
///
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::Timelike;

    use super::*;
    use crate::core::wishlist::Markets;

    #[test]
    fn test_date(){
        let limit = GameModel::get_expire_at();
        println!("{}",limit.to_string());
    }
}