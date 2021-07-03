/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
