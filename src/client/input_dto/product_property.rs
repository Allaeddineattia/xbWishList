use serde_json::{Value};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProductProperties{
    pub attributes: Option<Vec<Attribute>>,
    can_install_to_s_d_card: Option<bool>,
    category: Option<String>,
    sub_category: Option<String>,
    categories: Option<Vec<String>>,
    extensions: Option<Value>,
    is_accessible: Option<bool>,
    is_line_of_business_app: Option<bool>,
    is_demo: Option<bool>,
    is_published_to_legacy_windows_phone_store: Option<bool>,
    is_published_to_legacy_windows_store: Option<bool>,
    is_settings_app: Option<bool>,
    package_family_name: Option<String>,
    package_identity_name: Option<String>,
    publisher_certificate_name: Option<String>,
    publisher_id: String,
    xbox_live_tier: Option<String>,
    xbox_x_p_a: Value,
    xbox_cross_gen_set_id: Value,
    xbox_console_gen_optimized: Vec<String>,
    xbox_console_gen_compatible: Vec<String>,
    xbox_live_gold_required: Option<bool>,
    ownership_type: Value,
    pdp_background_color: Option<String>,
    has_add_ons: Option<bool>,
    revision_id: String,//could be date
    product_group_id: Option<String>,
    product_group_name: Option<String>,
    
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Attribute{
    pub name : String,
    pub minimum: Option<i32>,
    pub maximum: Option<i32>,
    pub applicable_platforms: Option<Vec<String>>,
    pub group: Option<Value>,
}


pub const PROPERTY_NAMES: phf::Map<&'static str, &'static str> = phf::phf_map!{
    "CapabilityXboxEnhanced" => "Xbox One X Enhanced",
    "Capability4k" => "4K Ultra HD",
    "XboxLive" => "Xbox Live",
    "CapabilityHDR" => "HDR10",
    "XPA" => "Xbox Play Anywhere",
    "SharedSplitScreen" => "Shared/Split Screen",
    "CrossPlatformMultiPlayer" => "Cross-platform multiPlayer",
    "CrossPlatformCoOp" => "Cross-Platform Co-op",
    "VREnabled" => "Windows Mixed Reality",
    "RayTracing" => "Ray Tracing",
    "60fps" => "60 fps+",
    "120fps" => "120 fps",
    "ConsoleGen9Optimized" => "Optimized for Series X|S",
    "GameStreaming" => "Cloud enabled",
    "ConsoleCrossGen" => "Smart Delivery",
    "ConsoleKeyboardMouse" => "Console Keyboard & Mouse",
    "PcGamePad" => "Pc Game Pad",
    "XboxLiveCrossGenMP" => "Xbox Live Cross-Gen Multiplayer",
    "XblOnlineMultiPlayer" => "Online Multiplayer",
    "XblLocalMultiPlayer" => "Xbox local multiPlayer",
    "XblLocalCoop" => "Xbox local co-op",
    "XblOnlineCoop" => "Xbox Live Cross-Gen Multiplayer",
};

