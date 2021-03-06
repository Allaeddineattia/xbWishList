use serde_json::{Value};
use serde::{Deserialize, Serialize};
use super::sku::Sku;
use super::availability::Availability;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplaySkuAvailability{
    pub sku: Option<Sku>,
    pub availabilities: Vec<Availability>,


}

