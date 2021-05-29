use mongodb::Database;
use mongodb::bson::{Document};

pub trait mongo_entity{
    fn to_entity(&self)-> Document;

}