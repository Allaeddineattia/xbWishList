use crate::core::purchase_option::PurchaseAvailability;
use mongodb::bson::{doc,Document};
use super::shared::MongoEntity;
use chrono::{DateTime, Utc};

impl MongoEntity for PurchaseAvailability {
    fn to_document(&self) -> Document{

        doc!{
            "sale_state" : self.sale_state_string(),
            "original_price" : self.original_price,
            "sale_price" : self.sale_price,
            "discount_ratio" : self.discount_ratio as u32,
            "currency": &(self.currency),
            "start_date": self.start_date,
            "end_date": self.end_date,
        }

    }

    
    fn create_from_document(doc : &Document) -> Self{
        let sale_state = Self::get_sale_state(doc.get_str("sale_state").unwrap());
        let original_price = doc.get_f64("original_price").unwrap();
        let sale_price = doc.get_f64("sale_price").unwrap();
        let discount_ratio = doc.get_i32("discount_ratio").unwrap() as u8;
        let currency: String = String::from(doc.get_str("currency").unwrap());
        let start_date: DateTime::<Utc> = *doc.get_datetime("start_date").unwrap();
        let end_date: DateTime::<Utc> = *doc.get_datetime("end_date").unwrap();
        PurchaseAvailability {
            sale_state,
            original_price,
            sale_price,
            discount_ratio,
            currency,
            start_date,
            end_date,
        }
    }
}