use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WishlistInfo{
    pub name: String,//Name
    pub games: Vec<WishlistInfoElement>,//Game
    pub language: String,//Language
    pub markets: Vec<String>,//Markets

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WishlistInfoElement{
    pub game: super::GameInfo,//Game
    pub markets: Vec<String>,//Markets

}