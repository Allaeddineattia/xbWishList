use crate::client::input_dto::catalog_response;
use crate::core::game::{Game, Property};
use crate::client::input_dto::product_property::{Attribute, ProductProperties};
pub fn map_game_attribute(attribute: &Attribute )-> Option<Property>
{
    let name = &attribute.name[..];
    match name {// string slice
        "CapabilityXboxEnhanced" => Some(Property::XboxOneXEnhanced),
        "Capability4k" => Some(Property::UltraHD4K),
        "XboxLive" => Some(Property::XboxLive),
        "CapabilityHDR" => Some(Property::HDR),
        "XPA" => Some(Property::XboxPlayAnywhere),
        "SharedSplitScreen" => Some(Property::SharedSplitScreen),
        "CrossPlatformMultiPlayer" => Some(Property::CrossPlatformMultiPlayer),
        "CrossPlatformCoOp" => Some(Property::CrossPlatformCoOp),
        "VREnabled" => Some(Property::WindowsMixedReality),
        "RayTracing" => Some(Property::RayTracing),
        "60fps" => Some(Property::FPS60),
        "120fps" => Some(Property::FPS120),
        "ConsoleGen9Optimized" => Some(Property::OptimizedForSeriesXAndS),
        "GameStreaming" => Some(Property::CloudEnabled),
        "ConsoleCrossGen" => Some(Property::SmartDelivery),
        "ConsoleKeyboardMouse" => Some(Property::ConsoleKeyboardMouse),
        "PcGamePad" => Some(Property::PcGamePad),
        "XboxLiveCrossGenMP" => Some(Property::CrossGenMultiPlayer),
        "XblOnlineMultiPlayer" => {
            let min = attribute.minimum.unwrap() as u16;
            let max = attribute.maximum.unwrap() as u16;
            Some(Property::OnlineMultiplayer(min, max))
        },
        "XblLocalMultiPlayer" => {
            let min = attribute.minimum.unwrap() as u16;
            let max = attribute.maximum.unwrap() as u16;
            Some(Property::LocalMultiplayer(min, max))
        },
        "XblLocalCoop" => {
            let min = attribute.minimum.unwrap() as u16;
            let max = attribute.maximum.unwrap() as u16;
            Some(Property::LocalCoop(min, max))
        },
        "XblOnlineCoop" => {
            let min = attribute.minimum.unwrap() as u16;
            let max = attribute.maximum.unwrap() as u16;
            Some(Property::OnlineCoop(min, max))
        },
        _ => {Option::None}
    }
}

pub fn get_properties(properties : & ProductProperties) -> Vec<Property>{
    properties
        .attributes
        .as_ref()
        .map(|attributes| {
            attributes
                .iter()
                .filter_map(map_game_attribute)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new)
}

pub fn create_game_from_product(product: &catalog_response::Product, language: &str ) -> Game
{
    let mut name = String::from("null");
    let mut developer_name = String::from("null");
    let mut publisher_name = String::from("null");
    let mut poster_uri = String::from("null");
    let mut description = String::from("null");
    for localized_properties in product.localized_properties.iter(){
        name = localized_properties.product_title.clone();
        description =  localized_properties.product_description.clone().unwrap_or(description.clone() );
        developer_name = localized_properties.developer_name.clone().unwrap_or(developer_name.clone() );
        publisher_name = localized_properties.publisher_name.clone().unwrap_or(publisher_name.clone() );
        poster_uri = localized_properties.images.iter()
            .find(|image| image.image_purpose == "Poster")
            .map(|image| format!("http:{}", &image.uri))
            .unwrap_or(poster_uri.clone() );
    };
    let properties = get_properties(&product.properties.as_ref().unwrap());
    Game::new(product.product_id.clone(), name, publisher_name, developer_name,
              poster_uri,  description, language.to_string(), properties)
}