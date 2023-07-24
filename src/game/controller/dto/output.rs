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
use utoipa::{ToSchema};
use chrono::{DateTime, Utc};
use crate::game::game::Game;
use crate::game::purchase_option::{PurchaseAvailability, PurchaseOption};
use crate::shared::date::my_date_format;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct GameResponse{
    pub id: String,// Id
    pub name: String,//Name
    pub publisher_name: String,//PublisherName
    pub developer_name: String,//DeveloperName
    pub description: String,//Description
    pub icon_u_r_l: String,//IconURL
    pub language: String,//Language
    pub purchase_options: Vec<PurchaseOptionResponse>//PurchaseOptions
}


impl GameResponse{
    pub fn new(game: Game) -> Self{
        let purchase_options: Vec<PurchaseOptionResponse> = game.purchase_options().into_iter().map(|map|{
            PurchaseOptionResponse::new(map.0.to_string(), map.1)
        }).collect();
        Self{
            id: game.id().to_string(),
            name: game.name().to_string(),
            description: game.description().description.to_string(),
            publisher_name: game.publisher().to_string(),
            developer_name: game.developer().to_string(),
            icon_u_r_l: game.poster_uri().to_string(),
            language: game.description_language().to_string(),
            purchase_options
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PurchaseOptionResponse {
    pub market: String,
    pub store_u_r_l: String,
    pub availabilities: Vec<AvailabilityResponse>,
}

impl PurchaseOptionResponse {
    pub fn new(market: String, option: &PurchaseOption) ->Self{
        Self{
            market,
            store_u_r_l: option.store_uri.to_string(),
            availabilities: option.purchase_availabilities.iter().map(AvailabilityResponse::new).collect()
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AvailabilityResponse {
    pub sale_state: String,
    pub original_price : f64,
    pub sale_price : f64,
    pub discount_ratio : u32,
    pub currency: String,
    #[serde(with = "my_date_format")]
    #[schema(example = "2023-07-13 20:00:00")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "my_date_format")]
    #[schema(example = "2023-07-13 20:00:00")]
    pub end_date: Option<DateTime<Utc>>,
}

impl AvailabilityResponse {
    pub fn new(availability: &PurchaseAvailability) -> Self {
        Self{
            sale_state: availability.sale_state_string().to_string(),
            original_price: availability.original_price,
            sale_price: availability.sale_price,
            discount_ratio: availability.discount_ratio as u32,
            currency: availability.currency.to_string(),
            start_date: Some(availability.start_date),
            end_date: Some(availability.end_date)
        }
    }
}