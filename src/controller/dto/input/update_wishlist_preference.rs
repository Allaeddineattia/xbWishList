use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateWishlistPreferenceDTO{
    pub name: String,//Name
    pub language: Option<String>,//Language
    pub markets: Option<Vec<String>>,//Markets
    pub games: Option<Vec<UpdateWishlistElement>>//Games
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateWishlistElement{
    pub id: String,//GameId
    pub markets: Vec<String>//Markets
}