use crate::repo::shared::{MongoEntity, UniqueEntity};
use mongodb::bson::{doc, Document, Bson};

pub struct WishlistModel{
    pub name: String,
    pub games: Vec<WishlistElementModel>,
    pub preference: WishlistPreferencesModel,
}

pub struct WishlistElementModel{
    pub game_id: String,
    pub markets: Vec<String>
}

pub struct WishlistPreferencesModel{
    pub language: String,
    pub markets: Vec<String>
}

impl MongoEntity for WishlistModel{
    fn to_document(&self) -> Document {
        let games : Vec<Document> = (&self.games).into_iter().map(|game|{game.to_document()}).collect();
        doc!{
            "name": &self.name,
            "games": games,
            "preference": &self.preference.to_document()
        }
    }

    fn from_document(doc: &Document) -> Self {
        let name = String::from(doc.get_str("name").unwrap());
        let games : Vec<WishlistElementModel> = doc.get_array("games").unwrap().into_iter().map(
            |bson|{
                match bson{
                    Bson::Document(document) => {
                        WishlistElementModel::from_document(document)
                    }
                    _ => {
                        panic!()
                    }
                }
            }
        ).collect();

        let preference = WishlistPreferencesModel::from_document(doc.get_document("preference").unwrap());
        Self{
            name,
            games,
            preference
        }
    }
}

impl UniqueEntity for WishlistModel{
    fn get_unique_selector(&self) -> Document {
        doc! {"name": &self.name}
    }
}

impl MongoEntity for WishlistElementModel{
    fn to_document(&self) -> Document {


        doc!{
            "game_id": &self.game_id,
            "markets": &self.markets,
            }

    }

    fn from_document(doc: &Document) -> Self {
        let game_id = String::from(doc.get_str("game_id").unwrap());
        let mut markets= Vec::<String>::new();
        if let Ok(markets_database) = doc.get_array("markets") {
            for bson in markets_database.into_iter(){
                match bson{
                    Bson::String(market)=>{
                        markets.push(market.clone());
                    },
                    _ =>{}
                }
            }
        }
        Self{
            game_id,
            markets
        }
    }
}

impl MongoEntity for WishlistPreferencesModel{
    fn to_document(&self) -> Document {
        doc!{
            "language": &self.language,
            "markets": &self.markets,

        }
    }

    fn from_document(doc: &Document) -> Self {
        let language = String::from(doc.get_str("language").unwrap());
        let mut markets= Vec::<String>::new();
        if let Ok(markets_database) = doc.get_array("markets"){
            for bson in markets_database.into_iter(){
                match bson{
                    Bson::String(market)=>{
                        markets.push(market.clone());
                    },
                    _ =>{}
                }
            }
        }

        Self{
            language,
            markets
        }

    }
}