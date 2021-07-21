use serde::{Deserialize, Serialize};
use crate::controller::dto::input::create_wishlist::WishlistElement;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddToWishlistDTO{
    pub name: String,//Name
    pub game: WishlistElement//Game
}