use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateWishlist{
    pub name: String,//Name
    pub language: String,//Language
    pub markets: Vec<String>,//Markets
    pub games: Vec<WishlistElement>//Games
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WishlistElement{
    pub id: String,//GameId
    pub markets: Option<Vec<String>>//Markets
}
