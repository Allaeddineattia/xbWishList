use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveFromWishlistDTO{
    pub name: String,//Name
    pub game_id: String//GameId
}