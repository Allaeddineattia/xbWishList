use crate::client::input_dto::catalog_response;
use super::purchase_option;
use crate::core::purchase_option::{PurchaseAvailability};
use std::collections::HashMap;


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



// TODO
// in relation games  

struct GameDescription{

    pub name: String,
    pub publisher: String,
    pub developer: String,
    pub description: String,
    pub poster_uri: String,
} 


impl GameDescription{
    fn new( name: String, publisher: String, developer: String, poster_uri: String, description: String)-> Self{
            GameDescription{
                 name, publisher, developer, poster_uri, description
            }
        } 

}

pub struct PurchaseOption{
    pub purchase_availabilities : Vec<PurchaseAvailability>,
    pub store_uri: String,
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




pub struct Game{
    pub id: String,
    pub game_description: (String, GameDescription),
    pub purchase_options: HashMap<String, PurchaseOption>,
    pub properties: Vec<Property>,
}

impl Game{

    pub fn new(id: String, name: String, publisher: String, developer: String, poster_uri: String,
               store_uri: String, description: String, language: String, properties: Vec<Property>) -> Self {
        let game_description = (language , GameDescription::new(name, publisher, developer, poster_uri, description));
        Game { id , game_description, purchase_options: HashMap::new(), properties}
    }


    pub fn add_purchase_option(&mut self, market: &str, store_uri: String, purchase_availabilities: Vec<PurchaseAvailability>){
        let purchase_option = PurchaseOption::new(purchase_availabilities, store_uri);
        self.purchase_options.insert(String::from(market), purchase_option);
    }

    pub fn id(&self) -> &str{ &self.id}
    pub fn name(&self) -> &str{ &self.game_description.1.name}
    pub fn publisher(&self) -> &str{&self.game_description.1.publisher}
    pub fn developer(&self) -> &str{&self.game_description.1.developer}
    pub fn description(&self) -> &str{&self.game_description.1.description}
    pub fn poster_uri(&self) -> &str{&self.game_description.1.poster_uri}
    pub fn description_language(&self) -> &str{&self.game_description.0}
    
    pub fn print(&self){
        println!("============");
        println!("  id:               {}", self.id);
        println!("  name:             {}", self.game_description.1.name);
        println!("  publisher_name:   {}", self.game_description.1.publisher);
        println!("  developer_name:   {}", self.game_description.1.developer);
        println!("  poster_uri:       {}", self.game_description.1.poster_uri);
        println!("  description:      {}", self.game_description.1.description);
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

