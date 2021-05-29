use crate::core::game::Game;
use super::shared;
use mongodb::{Database};
use mongodb::bson::{doc,Document, Array};

impl shared::mongo_entity for Game {
    fn to_entity(&self) -> Document{

        
        let vec: Vec<Document> = (&self.purchase_options).into_iter().map(
            |option| {
                doc!{
                    "sale_state" : option.sale_state_string(),
                    "original_price" : option.original_price,
                    "sale_price" : option.sale_price,
                    "discount_ratio" : option.discount_ratio as u32,
                    "currency": &option.currency,
                    "start_date": option.start_date.to_string(),
                    "end_date": option.end_date.to_string(),
                }
            }
        
        ).collect();
        doc!{
            "id" : &self.id,
            "name" : &self.name,
            "publisher" : &self.publisher,
            "poster_uri" : &self.poster_uri,
            "store_uri" : &self.store_uri,
            "purchase_options" : vec,

        }
    

    }
}
