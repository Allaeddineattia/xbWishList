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

use std::collections::{HashSet, HashMap};
use crate::game::Game;
use crate::game::xbox_api_client::markets::MARKETS;

pub struct Wishlist{
    pub name: String,
    pub games: HashMap<String, WishlistElement>,
    pub preference: WishlistPreferences,
}

pub struct WishlistElement{
    pub game: Game,
    pub markets: Markets
}

pub struct WishlistPreferences{
    pub language: String,
    pub markets_by_default: Markets
}

#[derive(Clone)]
pub struct Markets{
    markets: HashSet<String>,
}


impl Wishlist {
    pub fn new(name: String,  preference: WishlistPreferences, games: HashMap<String, WishlistElement>) -> Self{
        Wishlist{
            name,
            games,
            preference,
        }
    }

    pub fn games(&self) -> Vec<(&str, Vec<&str>)>{
        self.games.iter().map(|element|{
            (&element.0[..], element.1.markets.to_vec())
        }).collect()
    }

    pub fn add_a_game(& mut self, game: Game, markets: Option<Markets>) -> bool{
        let existing_game = self.games.get_mut(game.id());
        return if let Some(existing_game) = existing_game {
            if let Some(markets) = markets {
                if markets.equal(&existing_game.markets) {
                    return false;
                }
                existing_game.markets = markets;
                return true;
            }
            false
        } else {
            let game_markets: Markets;
            if let Some(markets) = markets {
                game_markets = markets;
            } else {
                game_markets = self.preference.markets_by_default.clone();
            }
            let element = WishlistElement::new(game, game_markets);
            self.games.insert(element.game.id().to_string(), element);
            true
        }
    }

    pub fn remove_a_game(&mut self, game_id:&str) -> bool{
        match  self.games.remove(game_id){
            None => {false}
            Some(_) => {true}
        }
    }

    pub fn preference(&self) -> &WishlistPreferences{
        &self.preference
    }
}

impl WishlistElement {
    pub fn new(game: Game, markets: Markets)-> Self{
        WishlistElement{
            game,
            markets
        }
    }
    pub fn markets(&self) -> Vec<&str>{
        self.markets.to_vec()
    }
}

impl WishlistPreferences{
    pub fn new(language: String, markets_by_default: Markets) -> Self {
        WishlistPreferences { language, markets_by_default }
    }

    pub fn markets(&self) -> Vec<&str>{
        self.markets_by_default.to_vec()
    }

}

impl Markets{
    pub fn from_vec_str(markets: Vec<String>) -> (Self, Vec<String>)//(Markets, InvalidMarkets)
    {
        let mut markets_hash = HashSet::<String>::new();
        let mut invalid_markets = Vec::<String>::new();
        for market in markets{
            let market_opt = MARKETS.get(&market);
            if let Some(_) = market_opt{
                markets_hash.insert(market);
            }else {
                invalid_markets.push(market);
            }
        }
        let markets = Self{
            markets: markets_hash
        };
        (markets, invalid_markets)
    }

    pub fn equal(&self,  rhs: &Self) -> bool{
        self.markets == rhs.markets
    }

    pub fn to_vec(&self) -> Vec<&str>{
        self.markets.iter().map(|s|{&s[..]}).collect()
    }


}