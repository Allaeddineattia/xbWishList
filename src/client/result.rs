use crate::client::catalog_response;

pub struct Result{
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub on_sale: bool,
    pub on_deals_with_gold: bool,
    pub poster_uri: String,
}

impl Result{

    pub fn print_price(product: &catalog_response::Product){
        for sku_availability in product.display_sku_availabilities.iter(){
            for availability in sku_availability.availabilities.iter(){
                println!("*********Availability*******");
                if let Some(condition) = &availability.conditions{
                    println!("start date: {}", condition.start_date.unwrap());
                    println!("end date: {}", condition.end_date.unwrap());
                }
                if let Some(order_managment) = &availability.order_management_data{
                    let price = order_managment.price.list_price;
                    let original_price = order_managment.price.m_s_r_p;
                    if price < original_price {
                        println!("___________OnSolde___________");
                        if let Some(properties) = &availability.properties{
                            if let Some(tags) = &properties.merchandising_tags{
                                for tag in tags.iter(){
                                    if tag == "LegacyDiscountGold" {
                                        println!("____With__Gold______")
                                    }
                                }
                            }
                        }
                    }
                    println!("list price: {}", order_managment.price.list_price);
                    println!("original price: {}", order_managment.price.m_s_r_p);
                    
                }
            }
        }
    }


    pub fn new(product: &catalog_response::Product) -> Result{
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
        
        Result{
            id: product.product_id.clone(),
            name: name,
            publisher: developper_name,
            on_deals_with_gold: false,
            on_sale: false,
            poster_uri: poster_uri,
        }
    }

    
    pub fn print(&self){
        println!("============");
        println!("  id:               {}", self.id);
        println!("  name:             {}", self.name);
        println!("  publisher_name:   {}", self.publisher);
        println!("  poster_uri:       {}", self.poster_uri);
        
    }
}
