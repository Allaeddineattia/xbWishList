use std::rc::Rc;
use mongodb::Database;
use crate::client::input_dto::catalog_response;
use crate::core::purchase_option::{PurchaseAvailability};
use crate::client::client_service::microsoft_api::XboxLiveLanguage;

pub struct PurchaseOptionService {
    db : Rc<Database>,
}

impl PurchaseOptionService{
    pub fn new(db: Rc<Database>) -> Self {
        PurchaseOptionService { db }
    }

    pub fn get_sales(&self,product: &catalog_response::Product) -> Vec<PurchaseAvailability>{
        let mut sales: Vec<PurchaseAvailability> = vec![];
        for sku_availability in product.display_sku_availabilities.iter()
        {
            for availability in sku_availability.availabilities.iter()
            {
                if availability.actions.iter().find(|&x| x == "Purchase") != None
                {
                    sales.push(PurchaseAvailability::new(availability));
                }
            }
        }
        sales
    }

}
