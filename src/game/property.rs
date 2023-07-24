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

use crate::game::xbox_api_client::input_dto::product_property::{Attribute, ProductProperties};

pub enum Property{
    XboxOneXEnhanced,
    UltraHD4K,
    XboxLive,
    HDR,
    XboxPlayAnywhere,
    SharedSplitScreen,
    CrossPlatformMultiPlayer,
    CrossPlatformCoOp,
    WindowsMixedReality,
    RayTracing,
    FPS60,
    FPS120,
    OptimizedForSeriesXAndS,
    CloudEnabled,
    SmartDelivery,
    ConsoleKeyboardMouse,
    PcGamePad,
    CrossGenMultiPlayer,
    OnlineMultiplayer(u16, u16),
    OnlineCoop(u16, u16),
    LocalMultiplayer(u16, u16),
    LocalCoop(u16, u16),
}

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