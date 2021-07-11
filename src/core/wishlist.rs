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

use std::collections::HashSet;
use crate::client::client_service::microsoft_api::{MARKETS};

pub struct WishlistPreferences{
    pub language: String,
    pub markets: HashSet<String>
}
impl WishlistPreferences{
    pub fn markets(&self) -> Vec<&str>{
        self.markets.iter().map(|s|{&s[..]}).collect()
    }
}

pub struct WishlistElement{

    pub game_id: String,
    pub markets: HashSet<String>
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

    pub fn markets(&self) -> Vec<&str>{
        self.markets.iter().map(|s|{&s[..]}).collect()
    }

}

pub struct Wishlist{
    pub name: String,
    pub games: Vec<WishlistElement>,
    pub preference: WishlistPreferences, 
}

impl Wishlist {
    pub fn new(name: &str,  preference: WishlistPreferences, games: &Vec<(&str, Option<HashSet<&str>>)>) -> Self{
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
            name: name.to_string(),
            games: game_list,
            preference, 
        }
    }
    pub fn games(&self) -> Vec<(&str, &HashSet<String>)>{
        self.games.iter().map(|element|{
            if element.markets.is_empty(){
                return (&element.game_id[..], &self.preference.markets);
            }else{
                return (&element.game_id[..], &element.markets);
            }
        }).collect()
    }

    pub fn preference(&self) -> &WishlistPreferences{
        &self.preference
    }
}