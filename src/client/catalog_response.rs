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

    // pub last_modified_date: String,
    // localized_properties: Vec<LocalizedProperty>,
    // market_properties: Vec<MarketProperty>,
    // product_a_schema: Option<String>,
    // product_b_schema: Option<String>,
    // product_id: String,
    // properties: Option<ProductProperties>,
    // alternate_ids: Option<Vec<String>>,
    // domain_data_version: Option<String>,
    // ingestion_source: Option<String>,
    // is_microsoft_product: Option<bool>,
    // preferred_sku_id: Option<String>,
    // product_type: Option<Value>,
    // validation_data: Option<ValidationData>,
    // merchandizing_tags: Option<Vec<Value>>,
    // part_d: Option<String>,
    // product_family: String,
    // schema_version: Option<String>,
    // product_kind: String,
    // display_sku_availabilities: Vec<DisplaySkuAvailability>,
    #[serde(with = "my_date_format")]
    pub last_modified_date: DateTime<Utc>,
    localized_properties: Value,
    market_properties: Value,
    product_a_schema: Value,
    product_b_schema: Value,
    product_id: Value,
    properties: Option<Value>,
    alternate_ids: Option<Vec<Value>>,
    domain_data_version: Value,
    ingestion_source: Value,
    is_microsoft_product: Option<bool>,
    preferred_sku_id: Value,
    product_type: Option<Value>,
    validation_data: Option<Value>,
    merchandizing_tags: Option<Vec<Value>>,
    part_d: Option<Value>,
    product_family: Value,
    schema_version: Value,
    product_kind: Value,
    display_sku_availabilities: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalizedProperty{
    // developer_name: Optional[str]
    // display_platform_properties: Optional[Any]
    // publisher_name: Optional[str]
    // publisher_website_uri: Optional[str]
    // support_uri: Optional[str]
    // eligibility_properties: Optional[Any]
    // franchises: Optional[List]
    // images: List[Image]
    // videos: Optional[List[Video]]
    // product_description: Optional[str]
    // product_title: str
    // short_title: Optional[str]
    // sort_title: Optional[str]
    // friendly_title: Optional[str]
    // short_description: Optional[str]
    // search_titles: Optional[List[SearchTitle]]
    // voice_title: Optional[str]
    // render_group_details: Optional[Any]
    // product_display_ranks: Optional[List]
    // interactive_model_config: Optional[Any]
    // interactive_3d_enabled: Optional[bool] = Field(alias="Interactive3DEnabled")
    // language: Optional[str]
    // markets: Optional[List[str]]
    developer_name: Value, 
    display_platform_properties: Value, 
    publisher_name: Value, 
    publisher_website_uri: Value, 
    support_uri: Value, 
    eligibility_properties: Value, 
    franchises: Value, 
    images: Value, 
    videos: Value, 
    product_description: Value, 
    product_title: Value, 
    short_title: Value, 
    sort_title: Value, 
    friendly_title: Value, 
    short_description: Value, 
    search_titles: Value, 
    voice_title: Value, 
    render_group_details: Value, 
    product_display_ranks: Value, 
    interactive_model_config: Value, 
    interactive_3d_enabled: Value,  
    language: Value, 
    markets: Value, 
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketProperty{
    // original_release_date: Optional[datetime]
    // original_release_friendly_name: Optional[str]
    // minimum_user_age: Optional[int]
    // content_ratings: Optional[List[ContentRating]]
    // related_products: Optional[List]
    // usage_data: List[UsageData]
    // bundle_config: Optional[Any]
    // markets: Optional[List[str]]

    original_release_date: Value, 
    original_release_friendly_name: Value, 
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
    // attributes: Optional[List]
    // can_install_to_sd_card: Optional[bool] = Field(alias="CanInstallToSDCard")
    // category: Optional[str]
    // sub_category: Optional[str]
    // categories: Optional[List[str]]
    // extensions: Any
    // is_accessible: Optional[bool]
    // is_line_of_business_app: Optional[bool]
    // is_published_to_legacy_windows_phone_store: Optional[bool]
    // is_published_to_legacy_windows_store: Optional[bool]
    // is_settings_app: Optional[bool]
    // package_family_name: Optional[str]
    // package_identity_name: Optional[str]
    // publisher_certificate_name: Optional[str]
    // publisher_id: str
    // xbox_live_tier: Any
    // xbox_xpa: Any = Field(alias="XboxXPA")
    // xbox_cross_gen_set_id: Any
    // xbox_console_gen_optimized: Any
    // xbox_console_gen_compatible: Any
    // xbox_live_gold_required: Optional[bool]
    // ownership_type: Any
    // pdp_background_color: Optional[str]
    // has_add_ons: Optional[bool]
    // revision_id: str
    // product_group_id: Optional[str]
    // product_group_name: Optional[str]
    

    attributes: Value,
    can_install_to_SD_card: Value, 
    category: Value,
    sub_category: Value,
    categories: Value,
    extensions: Value,
    is_accessible: Value,
    is_line_of_business_app: Value,
    is_published_to_legacy_windows_phone_store: Value,
    is_published_to_legacy_windows_store: Value,
    is_settings_app: Value,
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
    // sku: Optional[Sku]
    // availabilities: List[Availability]

    sku: Value, 
    availabilities: Value, 
}


