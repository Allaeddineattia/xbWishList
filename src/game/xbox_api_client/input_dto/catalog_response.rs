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
use super::localized_property::LocalizedProperty;
use super::market_property::MarketProperty;
use super::product_property::ProductProperties;
use super::sku_availability::DisplaySkuAvailability;
use crate::shared::date::my_date_format;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response{
    pub big_ids: Option<Vec<String>>,
    pub has_more_pages: Option<bool>,
    pub products: Vec<Product>,
    pub total_result_count: Option<u32>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Product{

    #[serde(with = "my_date_format")]
    pub last_modified_date: Option<DateTime<Utc>>,
    pub localized_properties: Vec<LocalizedProperty>,
    pub market_properties: Vec<MarketProperty>,
    pub product_a_schema: Option<String>,
    pub product_b_schema: Option<String>,
    pub product_id: String,
    pub properties: Option<ProductProperties>,
    alternate_ids: Option<Vec<AlternateId>>,
    domain_data_version: Option<String>,
    ingestion_source: Option<String>,
    is_microsoft_product: Option<bool>,
    preferred_sku_id: Option<String>,
    product_type: Option<String>,
    validation_data: Option<ValidationData>,
    merchandizing_tags: Option<Vec<Value>>,
    part_d: Option<String>,
    product_family: String,
    schema_version: Option<String>,
    product_kind: String,
    pub display_sku_availabilities: Vec<DisplaySkuAvailability>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidationData{
    passed_validation: bool,
    revision_id: String,
    validation_result_uri: Option<String>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlternateId{
    id_type: String,
    value: String,
}

