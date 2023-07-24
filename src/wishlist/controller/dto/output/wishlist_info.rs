use serde::{Deserialize, Serialize};
use crate::game::controller::GameResponse;

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
    pub game: GameResponse,//Game
    pub markets: Vec<String>,//Markets

}