use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};
use crate::wishlist::controller::dto::input::WishlistElement;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddToWishlistDTO{
    #[schema(example = "My Wishlist")]
    pub name: String,//Name
    #[schema(example = "00-B0-D0-63-C2-26")]
    pub owner_id: String,//OwnerId
    pub game: WishlistElement//Game
}