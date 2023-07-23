use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateWishlistPreferenceDTO{
    #[schema(example = "My Wishlist")]
    pub name: String,//Name
    #[schema(example = "en-US")]
    pub language: Option<String>,//Language
    #[schema(example = "[\"US\",\"BR\"]")]
    pub markets: Option<Vec<String>>,//Markets
    pub games: Option<Vec<UpdateWishlistElement>>//Games
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateWishlistElement{
    #[schema(example = "9MT5NJ5W7B8Z")]
    pub id: String,//GameId
    #[schema(example = "[\"US\",\"BR\"]")]
    pub markets: Vec<String>//Markets
}