use serde_json::{Value};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response{
    pub big_ids: Option<Vec<String>>,
    pub has_more_pages: Option<bool>,
    pub products: Vec<Product>,
    pub total_result_count: Option<u32>,

}

// enum gamepass{
//     xbox=9SJCZDHW896G ,
//     pc= 9SQ1C79LQTJJ,
//     ultimate= 9Q2FPGL45CQN
// }

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        println!("Serialize");
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        println!("deserialize");
        let s = String::deserialize(deserializer)?;
        println!("{}",s);
        let t = DateTime::parse_from_rfc3339(&s).unwrap();
        let ti: DateTime<Utc> = t.with_timezone(&Utc);
        Ok(ti)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Product{

    #[serde(with = "my_date_format")]
    pub last_modified_date: DateTime<Utc>,
    pub localized_properties: Vec<LocalizedProperty>,
    pub market_properties: Vec<MarketProperty>,
    pub product_a_schema: Option<String>,
    pub product_b_schema: Option<String>,
    pub product_id: String,
    properties: Option<ProductProperties>,
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
    display_sku_availabilities: Vec<DisplaySkuAvailability>,
}

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
    // videos: Option<Vec<Video>>
    // product_description: Option<String>
    pub product_title: String,
    // short_title: Option<String>
    // sort_title: Option<String>
    // friendly_title: Option<String>
    // short_description: Option<String>
    // search_titles: Option<Vec<SearchTitle>>
    // voice_title: Option<String>
    // render_group_details: Option<Value>
    // product_display_ranks: Option<Vec>
    // interactive_model_config: Option<Value>
    // interactive_3d_enabled: Option<bool> = Field(alias="interactive3DEnabled")
    // language: Option<String>
    // markets: Option<Vec<String>>



    //images: Value, 
    videos: Value, 
    product_description: Value, 
    short_title: Value, 
    sort_title: Value, 
    friendly_title: Value, 
    short_description: Value, 
    search_titles: Value, 
    voice_title: Value, 
    render_group_details: Value, 
    product_display_ranks: Value, 
    interactive_model_config: Value, 
    interactive_3_d_enabled: Value,  
    language: Value, 
    markets: Value, 

    
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
pub struct MarketProperty{
    // original_release_date: Option<datetime>
    original_release_friendly_name: Option<String>,
    // minimum_user_age: Option<i32>
    // content_ratings: Option<Vec<ContentRating>>
    // related_products: Option<Vec>
    // usage_data: Vec<UsageData>
    // bundle_config: Option<Value>
    // markets: Option<Vec<String>>

    original_release_date: Value,  
    minimum_user_age: Value, 
    content_ratings: Value, 
    related_products: Value, 
    usage_data: Value, 
    bundle_config: Value, 
    markets: Value, 
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductProperties{
    // attributes: Option<Vec>
    // can_install_to_sd_card: Option<bool> = Field(alias="CanInstallToSDCard")
    // category: Option<String>
    sub_category: Option<String>,
    // categories: Option<Vec<String>>
    // extensions: Value
    // is_accessible: Option<bool>
    // is_line_of_business_app: Option<bool>
    // is_published_to_legacy_windows_phone_store: Option<bool>
    // is_published_to_legacy_windows_store: Option<bool>
    is_settings_app: Option<bool>,
    // package_family_name: Option<String>
    // package_identity_name: Option<String>
    // publisher_certificate_name: Option<String>
    // publisher_id: String
    // xbox_live_tier: Value
    // xbox_xpa: Value = Field(alias="XboxXPA")
    // xbox_cross_gen_set_id: Value
    // xbox_console_gen_optimized: Value
    // xbox_console_gen_compatible: Value
    // xbox_live_gold_required: Option<bool>
    // ownership_type: Value
    // pdp_background_color: Option<String>
    // has_add_ons: Option<bool>
    // revision_id: String
    // product_group_id: Option<String>
    // product_group_name: Option<String>
    

    attributes: Value,
    can_install_to_s_d_card: Value, 
    category: Value,
    //sub_category: Value,
    categories: Value,
    extensions: Option<Value>,
    is_accessible: Value,
    is_line_of_business_app: Value,
    is_published_to_legacy_windows_phone_store: Value,
    is_published_to_legacy_windows_store: Value,
    //is_settings_app: Value,
    package_family_name: Value,
    package_identity_name: Value,
    publisher_certificate_name: Value,
    publisher_id: Value,
    xbox_live_tier: Value,
    xbox_XPA: Value, 
    xbox_cross_gen_set_id: Value,
    xbox_console_gen_optimized: Value,
    xbox_console_gen_compatible: Value,
    xbox_live_gold_required: Value,
    ownership_type: Value,
    pdp_background_color: Value,
    has_add_ons: Value,
    revision_id: Value,
    product_group_id: Value,
    product_group_name: Value,
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
pub struct DisplaySkuAvailability{
    // sku: Option<Sku>
    // availabilities: Vec<Availability>

    sku: Value, 
    availabilities: Value, 
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlternateId{
    id_type: String,
    value: String,
}

