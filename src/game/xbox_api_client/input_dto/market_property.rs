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
use chrono::{DateTime, Utc};
use crate::shared::date::my_date_format;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketProperty{
    #[serde(with = "my_date_format")]
    original_release_date: Option<DateTime<Utc>>,
    original_release_friendly_name: Option<String>,
    minimum_user_age: Option<i32>,
    content_ratings: Option<Vec<ContentRating>>,
    related_products: Option<Vec<RelatedProducts>>,
    usage_data: Vec<UsageData>,
    bundle_config: Option<Value>,
    markets: Option<Vec<String>>,
 
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContentRating{
    rating_system: String,
    rating_id: String,
    rating_descriptors: Vec<String>,
    rating_disclaimers: Vec<Value>,
    interactive_elements: Option<Vec<Value>>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RelatedProducts{
    related_product_id: String,
    relationship_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsageData{
    aggregate_time_span: String,
    average_rating: f32,
    play_count: Option<f32>,
    rating_count: f32,
    rental_count: Option<String>,
    trial_count: Option<String>,
    purchase_count: Option<String>,
}

