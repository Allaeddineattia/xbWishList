

use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResponse{
    results : Vec<SearchItem>,
    total_result_count: u32, //10 max 
} 

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchItem{
    product_family_name: String, //"Games"
    products: Vec<SearchItemProduct>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchItemProduct{
    background_color: String, //"#000000" could be empty
    height: u32, //1080
    width: u32, //1080
    image_type: String, // "BoxArt"
    platform_properties: Option<Vec<Value>>,
    icon: String ,
    product_id: String,
    r#type: String,
    title: String,
}
