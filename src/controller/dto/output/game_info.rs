use serde::{Deserialize, Serialize};
use serde_json;
use chrono::{DateTime, Utc};
use crate::client::shared::my_date_format;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameInfo{
    pub id: String,// Id
    pub name: String,//Name
    pub publisher_name: String,//PublisherName
    pub developer_name: String,//DeveloperName
    pub description: String,//Description
    pub icon_u_r_l: String,//IconURL
    pub language: String,//Language
    pub purchase_options: Vec<PurchaseOption>//PurchaseOptions
}

impl GameInfo{
    pub fn new(game: crate::core::game::Game) -> Self{
        let purchase_options: Vec<PurchaseOption> = game.purchase_options().into_iter().map(|map|{
           PurchaseOption::new(map.0.to_string(), map.1)
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurchaseOption{
    pub market: String,
    pub store_u_r_l: String,
    pub availabilities: Vec<Availability>,
}

impl PurchaseOption{
    pub fn new( market: String, option: &crate::core::game::PurchaseOption)->Self{
        Self{
            market,
            store_u_r_l: option.store_uri.to_string(),
            availabilities: option.purchase_availabilities.iter().map(Availability::new).collect()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Availability{
    pub sale_state: String,
    pub original_price : f64,
    pub sale_price : f64,
    pub discount_ratio : u32,
    pub currency: String,
    #[serde(with = "my_date_format")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "my_date_format")]
    pub end_date: Option<DateTime<Utc>>,
}

impl Availability{
    pub fn new(availability: &crate::core::purchase_option::PurchaseAvailability) -> Self {
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

