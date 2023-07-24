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

use std::collections::HashMap;
use crate::game::property::{get_properties, Property};
use crate::game::purchase_option::{PurchaseAvailability, PurchaseOption};
use crate::game::xbox_api_client::input_dto::catalog_response;

// TODO
// in relation games  
#[derive( Clone)]
pub struct GameDescription{

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

pub struct Game{
    id: String,
    game_description: (String, GameDescription),
    purchase_options: HashMap<String, PurchaseOption>,
    properties: Vec<Property>,
}

impl Game{

    pub fn new(id: String, name: String, publisher: String, developer: String, poster_uri: String,
               description: String, language: String, properties: Vec<Property>) -> Self {
        let game_description = (language , GameDescription::new(name, publisher, developer, poster_uri, description));
        Game { id , game_description, purchase_options: HashMap::new(), properties}
    }

    pub fn from_product(product: &catalog_response::Product, language: &str ) -> Self
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
        Self::new(product.product_id.clone(), name, publisher_name, developer_name,
                  poster_uri,  description, language.to_string(), properties)
    }

    pub fn create(id: String, game_description: (String, GameDescription), purchase_options: HashMap<String, PurchaseOption>) -> Self{
        Game{
            id, 
            game_description,
            purchase_options,
            properties: vec![],
        }
    }

    pub fn add_purchase_option(&mut self, market: &str, store_uri: String, purchase_availabilities: Vec<PurchaseAvailability>){
        let purchase_option = PurchaseOption::new(purchase_availabilities, store_uri);
        self.purchase_options.insert(String::from(market), purchase_option);
    }

    pub fn id(&self) -> &str{ &self.id}
    pub fn name(&self) -> &str{ &self.game_description.1.name}
    pub fn publisher(&self) -> &str{&self.game_description.1.publisher}
    pub fn developer(&self) -> &str{&self.game_description.1.developer}
    pub fn description(&self) -> &GameDescription {&self.game_description.1}
    pub fn poster_uri(&self) -> &str{&self.game_description.1.poster_uri}
    pub fn description_language(&self) -> &str{&self.game_description.0}


    #[allow(dead_code)]
    pub fn print(&self){
        println!("  id:               {}", self.id);
        println!("  name:             {}", self.game_description.1.name);
        println!("  publisher_name:   {}", self.game_description.1.publisher);
        println!("  developer_name:   {}", self.game_description.1.developer);
        println!("  poster_uri:       {}", self.game_description.1.poster_uri);
        println!("  description:      {}", self.game_description.1.description);
        self.print_purchase_options();
    }

    pub fn print_price(&self){
        println!("name:             {}", self.game_description.1.name);
        for option in &self.purchase_options {
            println!("market {}", option.0);
            option.1.print();
        }
    }

    fn print_purchase_options(&self){
        println!("_______Purchase__Options________");

        for option in &self.purchase_options {
            println!("Market {}", option.0);
            option.1.print();
        }

    }

    pub fn purchase_options(&self) -> &HashMap<String, PurchaseOption> {
        &self.purchase_options
    }
}

