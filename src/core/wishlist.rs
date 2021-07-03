use std::collections::HashSet;
use crate::client::client_service::microsoft_api::{MARKETS};

pub struct WishlistPreferences{
    pub language: String,
    pub markets: HashSet<String>
}

pub struct WishlistElement{
    game_id: String,
    markets: HashSet<String>
}

impl WishlistElement {

    pub fn new(game_id: String, markets: HashSet<String>)-> Self{
        WishlistElement{
            game_id,
            markets
        }
    }

    pub fn add_market(&mut self, market: String){
        let market_opt = MARKETS.get(&market);
        if let Some(_) = market_opt{
            self.markets.insert(market);
        }else {
            println!("market <{}> is not supported", market);
        }
        
    }

    pub fn remove_market(&mut self, market: &str) -> bool{
        self.markets.remove(market)
    }

    pub fn set_markets(&mut self, markets: HashSet<String>){
        self.markets = markets;
    }

}

pub struct Wishlist{
    id: String,
    games: Vec<WishlistElement>,
    wishlist_preference: WishlistPreferences, 
}
impl Wishlist {
    pub fn new(id: &str,  wishlist_preference: WishlistPreferences, games: &Vec<(&str, Option<HashSet<&str>>)>) -> Self{
        let mut game_list = Vec::<WishlistElement>::new();

        for game in games{
            let mut markets : HashSet<String> = HashSet::new();
            if let Some(markets_str) = &game.1{
                markets = markets_str.into_iter().map(|market_str|{
                    market_str.to_string()
                }).collect();
            }
            game_list.push(WishlistElement::new(game.0.to_string(), markets));
        }

        Wishlist{
            id: id.to_string(),
            games: game_list,
            wishlist_preference, 
        }
    }
    pub fn games(&self) -> Vec<(&str, &HashSet<String>)>{
        self.games.iter().map(|element|{
            if element.markets.is_empty(){
                return (&element.game_id[..], &self.wishlist_preference.markets);
            }else{
                return (&element.game_id[..], &element.markets);
            }
        }).collect()
    }

    pub fn wishlist_preference(&self) -> &WishlistPreferences{
        &self.wishlist_preference
    }
}