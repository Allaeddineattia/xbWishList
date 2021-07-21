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
use crate::client::client_service::microsoft_api::{MARKETS, XboxLiveLanguage};
use crate::core::game::Game;

pub struct Wishlist{
    pub name: String,
    pub games: Vec<WishlistElement>,
    pub preference: WishlistPreferences,
}

pub struct WishlistElement{
    pub game: Game,
    pub markets: Markets
}

pub struct WishlistPreferences{
    pub language: String,
    pub markets: Markets
}

#[derive(Clone)]
pub struct Markets{
    markets: HashSet<String>,
}

impl Wishlist {
    pub fn new(name: String,  preference: WishlistPreferences, games: Vec<WishlistElement>) -> Self{
        Wishlist{
            name,
            games,
            preference,
        }
    }

    pub fn games(&self) -> Vec<(&str, Vec<&str>)>{
        self.games.iter().map(|element|{
            return if element.markets.to_vec().is_empty() {
                (element.game.id(), self.preference.markets.to_vec())
            } else {
                (element.game.id(), element.markets.to_vec())
            }
        }).collect()
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
    pub fn new(language: String, markets: Markets) -> Self {
        WishlistPreferences { language, markets}
    }

    pub fn markets(&self) -> Vec<&str>{
        self.markets.to_vec()
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

    pub fn new()->Self{
        Self{
            markets: HashSet::<String>::new()
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

    pub fn to_vec(&self) -> Vec<&str>{
        self.markets.iter().map(|s|{&s[..]}).collect()
    }

    pub fn remove_market(&mut self, market: &str) -> bool{
        self.markets.remove(market)
    }

}