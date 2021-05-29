use crate::client::input_dto::catalog_response;
use super::purchase_option;

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








pub struct Game{
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub purchase_options: Vec<purchase_option::PurchaseAvailibility>,
    pub poster_uri: String,
    pub store_uri: String,
}

impl Game{

    fn print_price(&self){
        println!("_______Purchase__Options________");
        for option in &self.purchase_options {
            option.print();
        }
    }



    fn get_sales(product: &catalog_response::Product) -> Vec<purchase_option::PurchaseAvailibility>{
        let mut sales: Vec<purchase_option::PurchaseAvailibility> = vec![]; 
        for sku_availability in product.display_sku_availabilities.iter()
        {
            for availability in sku_availability.availabilities.iter()
            {
                if availability.actions.iter().find(|&x| x == "Purchase") != None 
                {
                    sales.push(purchase_option::PurchaseAvailibility::new(availability));
                }
            }
        }
        sales
    }

    pub fn new(product: &catalog_response::Product) -> Game{
        let mut name = String::from("null");
        let mut developper_name = String::from("null");
        let mut poster_uri = String::from("null");
        for localized_properties in product.localized_properties.iter(){
            name = localized_properties.product_title.clone();
            if let  Some(develop_name) = &localized_properties.developer_name{
                developper_name = develop_name.clone();
            }
            for image in localized_properties.images.iter(){
                if image.image_purpose == "Poster" {
                    let uri = String::from("http:") + &image.uri;
                    poster_uri = uri;
                }
            }

        }

        let store_uri = String::from("https://www.microsoft.com/") + "en-us" + "/p/" +
            &name.trim().replace(" ", "-").replace(":", "").replace("|", "").replace("&", "").to_lowercase() + "/" + &product.product_id;
        
        Game{
            id: product.product_id.clone(),
            name,
            publisher: developper_name,
            purchase_options: Game::get_sales(product),
            poster_uri,
            store_uri,
        }
    }

    
    pub fn print(&self){
        println!("============");
        println!("  id:               {}", self.id);
        println!("  name:             {}", self.name);
        println!("  publisher_name:   {}", self.publisher);
        println!("  poster_uri:       {}", self.poster_uri);
        println!("  store_uri:        {}", self.store_uri);
        Game::print_price(self);
    }

    
}

