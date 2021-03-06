use serde_json::{Value};
use serde::{Deserialize, Serialize};
use super::shared::my_date_format;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sku{
    #[serde(with = "my_date_format")]
    last_modified_date: Option<DateTime<Utc>>,
    localized_properties: Vec<SkuLocalizedProperty>,
    market_properties: Vec<SkuMarketProperty>,
    product_id: String,
    properties: SkuProperties,
    sku_a_schema: String,
    sku_b_schema: String,
    sku_id: String,
    sku_type: String,
    recurrence_policy: Value,
    subscription_policy_id: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkuLocalizedProperty{
    contributors: Option<Vec<Value>>,
    features: Option<Vec<Value>>,
    minimum_notes: Option<String>,
    recommended_notes: Option<String>,
    release_notes: Option<String>,
    display_platform_properties: Option<Value>,
    sku_description: String,
    sku_title: String,
    sku_button_title: Option<String>,
    delivery_date_overlay: Value,
    sku_display_rank: Option<Vec<Value>>,
    text_resources: Value,
    images: Option<Vec<Value>>,
    legal_text: Option<LegalText>,
    language: String,
    markets: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LegalText{
    additional_license_terms: String,
    copyright: String,
    copyright_uri: String,
    privacy_policy: String,
    privacy_policy_uri: String,
    tou: String,
    tou_uri: String,
}
    

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkuMarketProperty{
    #[serde(with = "my_date_format")]
    first_available_date: Option<DateTime<Utc>>,
    supported_languages: Option<Vec<String>>,
    package_ids: Option<Value>,
    pi_filter: Option<Value> ,
    markets: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkuProperties{
    early_adopter_enrollment_url: Value,
    fulfillment_data: Option<FulfillmentData>,
    fulfillment_type: Option<String>,
    fulfillment_plugin_id: Value,
    has_third_party_i_a_ps: Option<bool>,
    #[serde(with = "my_date_format")]
    last_update_date: Option<DateTime<Utc>>,
    hardware_properties: Option<HardwareProperties>,
    hardware_requirements: Option<Vec<Value>>,
    hardware_warning_list: Option<Vec<Value>>,
    installation_terms: String,
    packages: Option<Vec<Package>>,
    version_stringing: Option<String>,
    visible_to_b_2_b_service_ids: Vec<Value>,
    xbox_x_p_a: Option<bool>,
    bundled_skus: Option<Vec<Value>>,
    is_repurchasable: bool,
    sku_display_rank: i32,
    display_physical_store_inventory: Value,
    additional_identifiers: Vec<Value>,
    is_trial: bool,
    is_pre_order: bool,
    is_bundle: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FulfillmentData{
    product_id: String,
    wu_bundle_id: Option<String>,
    wu_category_id: String,
    package_family_name: String,
    sku_id: String,
    content: Value,
    package_features: Value,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HardwareProperties{
    minimum_hardware: Vec<Value>,
    recommended_hardware: Vec<Value>,
    minimum_processor: Value,
    recommended_processor: Value,
    minimum_graphics: Value,
    recommended_graphics: Value,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package{
    applications: Option<Vec<Application>>,
    architectures: Vec<String>,
    capabilities: Option<Vec<String>>,
    device_capabilities: Option<Vec<String>>,
    experience_ids: Option<Vec<Value>>,
    framework_dependencies: Option<Vec<FrameworkDependency>>,
    hardware_dependencies: Option<Vec<Value>>,
    hardware_requirements: Option<Vec<Value>>,
    hash: Option<String>,
    hash_algorithm: Option<String>,
    is_streaming_app: Option<bool>,
    languages: Option<Vec<String>>,
    max_download_size_in_bytes: u64,
    max_install_size_in_bytes: Option<u64>,
    package_format: String,
    package_family_name: Option<String>,
    main_package_family_name_for_dlc: Value,
    package_full_name: Option<String>,
    package_id: String,
    content_id: String,
    key_id: Option<String>,
    package_rank: Option<i32>,
    package_uri: Option<String>,
    platform_dependencies: Option<Vec<PlatformDependency>>,
    platform_dependency_xml_blob: Option<String>,
    resource_id: Option<String>,
    version: Option<String>,
    package_download_uris: Value,
    driver_dependencies: Option<Vec<Value>>,
    fulfillment_data: Option<FulfillmentData>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Application{
    application_id: String,
    declaration_order: i32,
    extensions: Vec<String>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FrameworkDependency{
    max_tested: i32,
    min_version: i32,
    package_identity: String,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformDependency{
    max_tested: Option<i32>,
    min_version: Option<i32>,
    platform_name: String,

}

