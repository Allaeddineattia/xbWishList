use serde::{Deserialize, Serialize};
use utoipa::{ToSchema };
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct CreateWishlist{
    #[schema(example = "My Wishlist")]
    pub name: String,//Name
    #[schema(example = "en-US")]
    pub language: String,//Language
    #[schema(example = "[\"US\",\"BR\"]")]
    pub markets: Vec<String>,//Markets
    pub games: Vec<WishlistElement>//Games
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct WishlistElement{
    #[schema(example = "9MT5NJ5W7B8Z")]
    pub id: String,//GameId
    #[schema(example = "[\"US\",\"BR\"]")]
    pub markets: Option<Vec<String>>//Markets
}
