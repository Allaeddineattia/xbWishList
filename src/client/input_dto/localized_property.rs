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

use serde_json::{Value};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalizedProperty{
    pub developer_name: Option<String>,
    pub display_platform_properties: Option<Value>,
    pub publisher_name: Option<String>,
    pub publisher_website_uri: Option<String>,
    pub support_uri: Option<String>,
    pub eligibility_properties: Option<Value>,// need some work
    pub franchises: Option<Vec<Value>>,
    pub images: Vec<Image>,
    pub videos: Option<Vec<Video>>,
    pub product_description: Option<String>,
    pub product_title: String,
    short_title: Option<String>,
    sort_title: Option<String>,
    friendly_title: Option<String>,
    short_description: Option<String>,
    search_titles: Option<Vec<SearchTitle>>,
    voice_title: Option<String>,
    render_group_details: Option<Value>,
    product_display_ranks: Option<Vec<Value>>,
    interactive_model_config: Option<Value>,
    interactive_3_d_enabled: Option<bool>,
    pub language: Option<String>,
    pub markets: Option<Vec<String>>,
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    file_id: Option<String>,
    e_i_s_listing_identifier: Value,
    background_color: Option<String>,
    caption: Option<String>,
    file_size_in_bytes: i32,
    foreground_color: Option<String>,
    pub height: i32,
    image_position_info: Option<String>,
    pub image_purpose: String,
    unscaled_image_s_h_a_256_hash: Option<String>,
    pub uri: String,
    pub width: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Video {
    pub uri: String,
    pub video_purpose: String,
    height: i32,
    width: i32,
    audio_encoding: String,
    video_encoding: String,
    video_position_info: String,
    caption: String,
    file_size_in_bytes: i32,
    pub preview_image: Image,
    sort_order: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchTitle{
    search_title_string: String,
    search_title_type: String,
}
