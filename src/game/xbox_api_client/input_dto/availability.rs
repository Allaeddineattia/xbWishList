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
use chrono::{DateTime, Utc};
use serde_json::{Value};
use crate::shared::date::my_date_format;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Availability{
    pub actions: Vec<String>,
    pub availability_a_schema: Option<String>,
    pub availability_b_schema: Option<String>,
    pub availability_id: Option<String>,
    pub conditions: Option<Conditions>,
    #[serde(with = "my_date_format")]
    pub last_modified_date: Option<DateTime<Utc>>,
    pub markets: Option<Vec<String>>,
    pub order_management_data: Option<OrderManagementData>,
    pub properties: Option<AvailabilityProperties>,
    pub sku_id: Option<String>,
    pub display_rank: Option<u64>,
    pub remediation_required: Option<bool>,
    pub remediations: Option<Vec<Remediation>>,
    pub licensing_data: Option<LicensingData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Remediation{
    pub remediation_id : String,
    pub r#type : String,
    pub big_id : String,

}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Conditions{
    pub client_conditions: ClientConditions,
    #[serde(with = "my_date_format")]
    pub end_date: Option<DateTime<Utc>>,
    pub resource_set_ids: Vec<String>,
    #[serde(with = "my_date_format")]
    pub start_date: Option<DateTime<Utc>>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientConditions{
    pub allowed_platforms: Vec<AllowedPlatform>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllowedPlatform{
    pub max_version: Option<u64>,
    pub min_version: Option<u64>,
    pub platform_name: String,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderManagementData{
    pub granted_entitlement_keys: Option<Vec<Value>>,
    pub p_i_filter: Option<PIFilter>,
    pub price: Price,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PIFilter{
    pub exclusion_properties: Vec<Value>,
    pub inclusion_properties: Vec<Value>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Price{
    pub currency_code: String,
    pub is_p_i_required: bool,
    pub list_price: f64,
    pub m_s_r_p: f64,
    pub tax_type: String,
    pub wholesale_currency_code: String,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvailabilityProperties{
    //#[serde(with = "my_date_format")]
    original_release_date: Option<Value>,
    pub merchandising_tags: Option<Vec<String>>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LicensingData{
    pub satisfying_entitlement_keys: Vec<SatisfyingEntitlementKey>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SatisfyingEntitlementKey{
    pub entitlement_keys: Vec<String>,
    pub licensing_key_ids: Vec<String>,
}