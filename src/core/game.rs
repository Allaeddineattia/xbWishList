use crate::client::input_dto::catalog_response;
use super::purchase_option;
use crate::core::purchase_option::{PurchaseAvailability, PurchaseOption};
/*
mod remediaition_values{
    pub const XBOX_GAME_PASS: &str      = "9SJCZDHW896G";
    pub const PC_GAME_PASS: &str        = "9SQ1C79LQTJJ";
    pub const GAME_PASS_ULTIMATE: &str  = "9Q2FPGL45CQN";
    pub const XBOX_LIVE_GOLD: &str      = "9ZH7BH6P9RM7";
    pub const EA_PLAY: &str             = "9N8KCDNKJJQ6";

}

mod affirmation_values{
    pub const XBOX_GAME_PASS: &str = "9WNZS2ZC9L74";
    pub const PC_GAME_PASS: &str   = "9VP428G6BQ82";
    pub const XBOX_LIVE_GOLD: &str = "9RVBF5P99P15";
}

*/






pub struct Game{
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub developer: String,
    pub purchase_options: Vec<PurchaseOption>,
    pub poster_uri: String,
    pub store_uri: String,
}

impl Game{

    pub fn new(id: String, name: String, publisher: String, developer: String,
               purchase_options: Vec<PurchaseOption>, poster_uri: String, store_uri: String) -> Self {
        Game { id, name, publisher, developer, purchase_options, poster_uri, store_uri }
    }


    
    pub fn print(&self){
        println!("============");
        println!("  id:               {}", self.id);
        println!("  name:             {}", self.name);
        println!("  publisher_name:   {}", self.publisher);
        println!("  developer_name:   {}", self.developer);
        println!("  poster_uri:       {}", self.poster_uri);
        println!("  store_uri:        {}", self.store_uri);
        Game::print_price(self);
    }



}

