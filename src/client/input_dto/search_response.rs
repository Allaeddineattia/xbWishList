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
    pub results : Vec<SearchItem>,
    pub total_result_count: u32, //10 max 
} 

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchItem{
    pub product_family_name: String, //"Games"
    pub products: Vec<SearchItemProduct>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchItemProduct{
    pub background_color: String, //"#000000" could be empty
    pub height: u32, //1080
    pub width: u32, //1080
    pub image_type: String, // "BoxArt"
    pub platform_properties: Option<Vec<Value>>,
    pub icon: String,//url
    pub product_id: String,
    pub r#type: String,
    pub title: String,
}
