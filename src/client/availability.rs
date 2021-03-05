use serde_json::{Value};
use serde::{Deserialize, Serialize};
use super::shared::my_date_format;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Availability{
    actions: Vec<String>,
    availability_a_schema: Option<String>,
    availability_b_schema: Option<String>,
    availability_id: Option<String>,
    conditions: Option<Conditions>,
    #[serde(with = "my_date_format")]
    last_modified_date: Option<DateTime<Utc>>,
    markets: Option<Vec<String>>,
    order_management_data: Option<OrderManagementData>,
    properties: Option<AvailabilityProperties>,
    sku_id: Option<String>,
    display_rank: Option<u64>,
    remediation_required: Option<bool>,
    licensing_data: Option<LicensingData>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Conditions{
    client_conditions: ClientConditions,
    #[serde(with = "my_date_format")]
    end_date: Option<DateTime<Utc>>,
    resource_set_ids: Vec<String>,
    #[serde(with = "my_date_format")]
    start_date: Option<DateTime<Utc>>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientConditions{
    allowed_platforms: Vec<AllowedPlatform>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllowedPlatform{
    max_version: Option<u64>,
    min_version: Option<u64>,
    platform_name: String,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderManagementData{
    granted_entitlement_keys: Option<Vec<Value>>,
    p_i_filter: Option<PIFilter>,
    price: Price,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PIFilter{
    exclusion_properties: Vec<Value>,
    inclusion_properties: Vec<Value>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Price{
    currency_code: String,
    is_p_i_required: bool,
    list_price: f32,
    m_s_r_p: f32,
    tax_type: String,
    wholesale_currency_code: String,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvailabilityProperties{
    //#[serde(with = "my_date_format")]
    original_release_date: Option<Value>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LicensingData{
    satisfying_entitlement_keys: Vec<SatisfyingEntitlementKey>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SatisfyingEntitlementKey{
    entitlement_keys: Vec<String>,
    licensing_key_ids: Vec<String>,
}