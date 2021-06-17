use crate::client::input_dto::catalog_response;
use super::purchase_option;
use crate::core::purchase_option::{PurchaseAvailability};
use std::collections::HashMap;
/*
mod remediaition_values{
    pub const XBOX_GAME_PASS: &str      = "9SJCZDHW896G";
    pub const PC_GAME_PASS: &str        = "9SQ1C79LQTJJ";
    pub const GAME_PASS_ULTIMATE: &str  = "9Q2FPGL45CQN";
    pub const XBOX_LIVE_GOLD: &str      = "9ZH7BH6P9RM7";
    pub const EA_PLAY: &str             = "9N8KCDNKJJQ6";

}

mod affirmation_values{
    pub const XBOX_GAME_PASS: &str = "9WNZS2ZC9L74";
    pub const PC_GAME_PASS: &str   = "9VP428G6BQ82";
    pub const XBOX_LIVE_GOLD: &str = "9RVBF5P99P15";
}

*/

mod properties{
    pub const XboxOneXEnhanced: (&str, &str) = ("CapabilityXboxEnhanced", "Xbox One X Enhanced");
    pub const UltraHD_4K: (&str, &str) = ("Capability4k", "4K Ultra HD");
    pub const XboxLive: (&str, &str) = ("XboxLive", "Xbox Live");
    pub const HDR: (&str, &str) = ("CapabilityHDR", "HDR10");
    pub const XboxPlayAnywhere: (&str, &str) = ("XPA", "Xbox Play Anywhere");
    pub const Shared_SplitScreen: (&str, &str) = ( "SharedSplitScreen", "Shared/Split Screen");
    pub const Cross_PlatformMultiPlayer: (&str, &str) = ("CrossPlatformMultiPlayer", "Cross-platform multiPlayer");
    pub const CrossPlatformCo_Op: (&str, &str) = ("CrossPlatformCoOp", "Cross-Platform Co-op");
    pub const WindowsMixedReality: (&str, &str) = ("VREnabled", "Windows Mixed Reality");
    pub const RayTracing: (&str, &str) = ("RayTracing", "Ray Tracing");
    pub const FPS60: (&str, &str) = ("60fps", "60 fps+");
    pub const FPS120: (&str, &str) = ("120fps", "120 fps");
    pub const OptimizedForSeriesXAndS: (&str, &str) = ("ConsoleGen9Optimized", "Optimized for Series X|S");
    pub const CloudEnabled: (&str, &str) = ("GameStreaming" ,"Cloud enabled");
    pub const SmartDelivery: (&str, &str) = ("ConsoleCrossGen", "Smart Delivery");
    pub const ConsoleKeyboardMouse: (&str, &str) = ("ConsoleKeyboardMouse", "Console Keyboard & Mouse");
    pub const PcGamePad: (&str, &str) = ("PcGamePad" ,"Pc Game Pad");
    pub const XboxLiveCrossGenMultiPlayer: (&str, &str) = ("XboxLiveCrossGenMP","Xbox Live Cross-Gen Multiplayer");
}

enum Property{
    XboxOneXEnhanced,
    UltraHD_4K,
    XboxLive,
    HDR,
    XboxPlayAnywhere,
    Shared_SplitScreen,
    Cross_PlatformMultiPlayer, 
    CrossPlatformCo_Op,
    WindowsMixedReality,
    RayTracing,
    FPS60,
    FPS120,
    OptimizedForSeriesXAndS,
    CloudEnabled,
    SmartDelivery,
    ConsoleKeyboardMouse,
    PcGamePad,
    XboxLiveCrossGenMultiPlayer,
}



// TODO
// in relation games  

struct game_discription{

    pub name: String,
    pub publisher: String,
    pub developer: String,
    pub description: String,
    pub poster_uri: String,
    pub store_uri: String,
} 


impl game_discription{
    fn new( name: String, publisher: String, developer: String, poster_uri: String,
        store_uri: String, description: String)-> Self{
            game_discription{
                 name, publisher, developer, poster_uri, store_uri, description
            }
        } 

}

pub struct PurchaseOption{
    purchase_availabilities : Vec<PurchaseAvailability>,
    store_uri: String,
}

impl PurchaseOption{
    fn new(purchase_availabilities: Vec<PurchaseAvailability>, store_uri: String,)->Self{
        PurchaseOption{
            purchase_availabilities,
            store_uri,
        }
    }
    fn print(&self){
        println!("store_uri {}", self.store_uri);
        for availability in self.purchase_availabilities.iter(){
            availability.print();
        }
    }
}

enum Property{
    
    XboxOneXEnhanced, //CapabilityXboxEnhanced
    _4KUltraHD, //Capability4k
    XboxLive, //XboxLive
    HDR10, //CapabilityHDR,
    XboxPlayAnywhere, // XPA
    Shared_SplitScreen, //SharedSplitScreen
    Cross_PlatformMultiPlayer, //CrossPlatformMultiPlayer
    CrossPlatformCo_Op,//CrossPlatformCoOp
    WindowsMixedReality,//VREnabled
    RayTracing, // RayTracing
    OptimizedForSeriesX|S, //ConsoleGen9Optimized,
    60fps+, //60fps 
    120fps, //120fps
    CloudEnabled, //GameStreaming
    SmartDelivery, //ConsoleCrossGen
    ConsoleKeyboardMouse, //ConsoleKeyboardMouse
    PcGamePad, //PcGamePad
    XboxLiveCrossGenMultiPlayer, //XboxLiveCrossGenMP
} 


pub struct Game{
    pub id: String,
    pub game_discription: (String, game_discription),
    pub purchase_options: HashMap<String, PurchaseOption>,
    pub properties: Vec<Property>,
}

impl Game{

    pub fn new(id: String, name: String, publisher: String, developer: String, poster_uri: String,
               store_uri: String, description: String, language: String) -> Self {
        let game_discription = (language , game_discription::new(name, publisher, developer, poster_uri, store_uri, description));
        Game { id , game_discription, purchase_options: HashMap::new()}
    }


    pub fn add_purchase_option(&mut self, market: &str, store_uri: &str, purchase_availabilities: Vec<PurchaseAvailability>){
        let purchase_option = PurchaseOption::new(purchase_availabilities, store_uri.to_string());
        self.purchase_options.insert(String::from(market), purchase_option);
    }

    pub fn id(&self) -> &str{ &self.id}
    pub fn name(&self) -> &str{ &self.game_discription.1.name}
    pub fn publisher(&self) -> &str{&self.game_discription.1.publisher}
    pub fn developer(&self) -> &str{&self.game_discription.1.developer}
    pub fn description(&self) -> &str{&self.game_discription.1.description}
    pub fn poster_uri(&self) -> &str{&self.game_discription.1.poster_uri}
    pub fn store_uri(&self) -> &str{&self.game_discription.1.store_uri} 
    
    pub fn print(&self){
        println!("============");
        println!("  id:               {}", self.id);
        println!("  name:             {}", self.game_discription.1.name);
        println!("  publisher_name:   {}", self.game_discription.1.publisher);
        println!("  developer_name:   {}", self.game_discription.1.developer);
        println!("  poster_uri:       {}", self.game_discription.1.poster_uri);
        println!("  store_uri:        {}", self.game_discription.1.store_uri);
        println!("  description:        {}", self.game_discription.1.description);
        self.print_price();
    }

    fn print_price(&self){
        println!("_______Purchase__Options________");

        for option in &self.purchase_options {
            println!("Market {}", option.0);
            option.1.print();
        }

    }


}

