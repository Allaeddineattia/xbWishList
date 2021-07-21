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
use crate::{core::wishlist::Wishlist, repo::shared::Repo};
use std::rc::Rc;
use mongodb::Database;
use crate::repo::wishlist_repo;
use std::sync::Arc;

pub struct WishlistService{
    game_service: Arc<GameService>,
    wishlist_repo: Arc<wishlist_repo::WishlistRepo>,
}

impl  WishlistService {

    pub fn new(game_service: Arc<GameService>, wishlist_repo: Arc<wishlist_repo::WishlistRepo>) -> Self {
        WishlistService { game_service, wishlist_repo }
    }

    pub async fn print_wishlist(&self, wishlist: &Wishlist){
        let pref = wishlist.preference();
        for game in wishlist.games(){

            self.game_service.get_game_info(game.0, &pref.language, game.1.into_iter().map(|string|{
                &string[..]
            }).collect()).await;
        }
    }


    pub async fn save(&self, wishlist: &Wishlist){
        self.wishlist_repo.save_wishlist(wishlist).await;
        if let Some (wishlist_result) = self.wishlist_repo.fetch_by_name(&wishlist.name).await{
            println!("Element saved correctly");
        }else {
            println!("couldn't save the file ")
        }
    } 

    pub async fn get_wishlist(&self, name: &str) -> Option<Wishlist>{
        self.wishlist_repo.fetch_by_name(name).await
    }

}

