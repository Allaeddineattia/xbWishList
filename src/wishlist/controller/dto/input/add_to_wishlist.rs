use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};
use crate::wishlist::controller::dto::input::WishlistElement;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddToWishlistDTO{
    #[schema(example = "My Wishlist")]
    pub name: String,//Name
    pub game: WishlistElement//Game
}