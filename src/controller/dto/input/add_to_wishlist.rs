use serde::{Deserialize, Serialize};
use crate::controller::dto::input::create_wishlist::WishlistElement;
use utoipa::{ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddToWishlistDTO{
    #[schema(example = "My Wishlist")]
    pub name: String,//Name
    pub game: WishlistElement//Game
}