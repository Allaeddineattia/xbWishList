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
use super::game_service::GameService;
use crate::core::wishlist::Wishlist;
use std::rc::Rc;
pub struct WishlistService{
    game_service: Rc<GameService>
}

impl  WishlistService {
    pub fn new(game_service: Rc<GameService>) -> Self{
        WishlistService{
            game_service
        }
    }
    pub async fn print_wishlist(&self, wishlist: &Wishlist){
        let pref = wishlist.wishlist_preference();
        for game in wishlist.games(){

            self.game_service.get_game_info(game.0, &pref.language, game.1.into_iter().map(|string|{
                &string[..]
            }).collect()).await;
        }
    }
}

