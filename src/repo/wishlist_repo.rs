
/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::{core::wishlist, repo::shared::Repo};
use mongodb::bson::{doc, Document, Bson};
use super::shared;

use std::collections::{HashMap, HashSet};
use mongodb::{Collection, Database};

impl shared::MongoEntity for wishlist::WishlistElement {
    fn to_document(&self)-> Document{
        doc!{
            "game_id": &self.game_id,
            "markets": self.markets(),
            
        }
    }

    fn create_from_document(doc : &Document) -> Self{
        let game_id = String::from(doc.get_str("game_id").unwrap());
        let mut markets: HashSet<String> = HashSet::new();
        for bson in doc.get_array("markets").unwrap().into_iter(){
            match bson{
                Bson::String(market)=>{
                    markets.insert(market.clone());
                },
                _ =>{}
            }
        };
        Self::new(game_id, markets)
    }

}

impl shared::MongoEntity for wishlist::WishlistPreferences{
    fn to_document(&self)-> Document{
        doc!{
            "language": &self.language,
            "markets": self.markets(),
            
        }
    }

    fn create_from_document(doc : &Document) -> Self{
        let language = String::from(doc.get_str("language").unwrap());
        let mut markets: HashSet<String> = HashSet::new();
        for bson in doc.get_array("markets").unwrap().into_iter(){
            match bson{
                Bson::String(market)=>{
                    markets.insert(market.clone());
                },
                _ =>{}
            }
        };
        Self{
            language,
            markets
        }
        
    }
}

impl shared::MongoEntity for wishlist::Wishlist{

    fn to_document(&self)-> Document{
        let games : Vec<Document> = (&self.games).into_iter().map(|game|{game.to_document()}).collect();
        doc!{
            "name": &self.name,
            "games": games,
            "preference": &self.preference().to_document()

        }
    }
    fn create_from_document(doc : &Document) -> Self{
        let name = String::from(doc.get_str("name").unwrap());
        let games : Vec<wishlist::WishlistElement> = doc.get_array("games").unwrap().into_iter().map(
            |bson|{
                match bson{
                    Bson::Document(document) => {
                        wishlist::WishlistElement::create_from_document(document)
                    }
                    _ => {
                        panic!()
                    }
                }

            }
        ).collect();
        let preference = doc.get_document("preference").unwrap();
        let preference = wishlist::WishlistPreferences::create_from_document(preference);
        Self{
            name,
            games,
            preference
        }
        
    }
}

impl shared::UniqueEntity for wishlist::Wishlist{
    fn get_unique_selector(&self) -> Document {
        doc! {"name": &self.name}
    }
}


pub struct  WishlistRepo{
    data_base_collection : Collection,
    collection_name : String ,
}

impl WishlistRepo {
    pub fn new(data_base : & Database) -> Self{
        let collection_name = String::from("wishlist");
        let data_base_collection = data_base.collection(&collection_name);
        Self{
            collection_name,
            data_base_collection
        }
    }

    pub async fn fetch_by_name(&self, name: &str) -> Option<wishlist::Wishlist>{
        let query = doc! {"name": name};
        self.fetch_by_query(query).await
    } 
}

impl shared::Repo<wishlist::Wishlist> for WishlistRepo{
    fn get_data_base_collection(&self) -> & Collection {
        & self.data_base_collection
    }
    fn get_collection_name(&self) -> & str{
        & self.collection_name
    }

}
