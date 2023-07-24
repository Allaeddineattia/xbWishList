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

use mongodb::bson::{doc,Document};
use chrono::{DateTime, Utc};
use crate::game::purchase_option::PurchaseAvailability;
use crate::shared::repository::MongoEntity;

pub struct PurchaseAvailabilityRepo;

impl PurchaseAvailabilityRepo{
    pub fn new() -> Self {
        PurchaseAvailabilityRepo {}
    }
}

impl MongoEntity for PurchaseAvailability{
    fn to_document(&self) -> Document {

        doc!{
            "id" : &self.id,
            "sale_state" : self.sale_state_string(),
            "original_price" : self.original_price,
            "sale_price" : self.sale_price,
            "discount_ratio" : self.discount_ratio as u32,
            "currency": &(self.currency),
            "start_date": mongodb::bson::DateTime::from_chrono(self.start_date),
            "end_date": mongodb::bson::DateTime::from_chrono(self.end_date),
        }
    }

    fn from_document(doc: &Document) -> PurchaseAvailability {
        let id = String::from(doc.get_str("id").unwrap());
        let sale_state = Self::get_sale_state(doc.get_str("sale_state").unwrap());
        let original_price = doc.get_f64("original_price").unwrap();
        let sale_price = doc.get_f64("sale_price").unwrap();
        let discount_ratio = doc.get_i32("discount_ratio").unwrap() as u8;
        let currency: String = String::from(doc.get_str("currency").unwrap());
        let start_date: DateTime::<Utc> = (*doc.get_datetime("start_date").unwrap()).into();
        let end_date: DateTime::<Utc> = (*doc.get_datetime("end_date").unwrap()).into();
        PurchaseAvailability {
            id,
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
