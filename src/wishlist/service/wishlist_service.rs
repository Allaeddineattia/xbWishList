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
use std::sync::Arc;
use crate::game::GameService;
use crate::wishlist::repo::wishlist_repo::WishlistRepo;
use crate::wishlist::wishlist::Wishlist;

pub struct WishlistService{
    game_service: Arc<GameService>,
    wishlist_repo: Arc<WishlistRepo>,
}

impl  WishlistService {

    pub fn new(game_service: Arc<GameService>, wishlist_repo: Arc<WishlistRepo>) -> Self {
        WishlistService { game_service, wishlist_repo }
    }

    pub async fn print_wishlist(&self, wishlist: &Wishlist){
        let pref = wishlist.preference();
        for game in wishlist.games(){

            self.game_service.get_game_info(game.0, &pref.language, &game.1.into_iter().map(|string|{
                &string[..]
            }).collect()).await;
        }
    }

    pub async fn save(&self, wishlist: &Wishlist){
        self.wishlist_repo.save_wishlist(wishlist).await;
        if let Some (_) = self.wishlist_repo.fetch_by_name(&wishlist.name, &wishlist.owner_id).await{
            println!("Element saved correctly");
        }else {
            println!("couldn't save the file ")
        }
    }

    pub async fn get_wishlist(&self, name: &str, owner_id: &str) -> Option<Wishlist>{
        self.wishlist_repo.fetch_by_name(name, owner_id).await
    }

    pub async fn get_all(&self, owner_id: &str) -> Vec<Wishlist>{
        self.wishlist_repo.get_all( owner_id ).await
    }

    pub async fn delete(&self, name: &str, owner_id: &str)-> bool {
        self.wishlist_repo.delete_by_name(name, owner_id).await
    }

}

