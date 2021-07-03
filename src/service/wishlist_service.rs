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

