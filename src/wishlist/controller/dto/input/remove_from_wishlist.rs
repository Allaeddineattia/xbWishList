use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveFromWishlistDTO{
    #[schema(example = "My Wishlist")]
    pub name: String,//Name
    #[schema(example = "9MT5NJ5W7B8Z")]
    pub game_id: String//GameId
}