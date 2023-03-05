
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetGameInfo{
    pub id: String,// Id
    pub language: String,//Languages
    pub markets: Vec<String>//Markets
}