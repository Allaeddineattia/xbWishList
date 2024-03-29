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

use super::xbox_api_client::input_dto::availability;
use chrono::{DateTime, Utc, NaiveDateTime};


#[derive(Clone)]
pub struct PurchaseOption{
    pub purchase_availabilities : Vec<PurchaseAvailability>,
    pub store_uri: String,
}

impl PurchaseOption{
    pub fn new(purchase_availabilities: Vec<PurchaseAvailability>, store_uri: String,)->Self{
        PurchaseOption{
            purchase_availabilities,
            store_uri,
        }
    }
    pub(crate) fn print(&self){
        //println!("store_uri {}", self.store_uri);
        println!("{{");
        for availability in self.purchase_availabilities.iter(){
            availability.print();
        }
        println!("}}\n");
    }

}

mod remediation_values {
    pub const XBOX_GAME_PASS: &str      = "9SJCZDHW896G";
    pub const PC_GAME_PASS: &str        = "9SQ1C79LQTJJ";
    pub const GAME_PASS_ULTIMATE: &str  = "9Q2FPGL45CQN";
    pub const XBOX_LIVE_GOLD: &str      = "9ZH7BH6P9RM7";
    pub const EA_PLAY: &str             = "9N8KCDNKJJQ6";

}
#[derive(Clone)]
pub enum SaleState{
    PublicSale,
    DealsWithGold,
    DealsWithXboxGP,
    DealsWithPcGP,
    DealsWithGPUltimate,
    DealsWithEAPlay,
    NotOnSale,
}

#[derive(Clone)]
pub struct PurchaseAvailability {
    pub id: String,
    pub sale_state: SaleState,
    pub original_price : f64,
    pub sale_price : f64,
    pub discount_ratio : u8,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl PurchaseAvailability {

    fn get_sale_ratio(new_price:f64, old_price:f64) -> u8{
        let ratio = (new_price * 100.0 / old_price).round() as u8;
        return 100 - ratio;
    }

    pub fn get_sale_state(sale_state: &str) -> SaleState{
        match sale_state {
            "PublicSale" => SaleState::PublicSale,
            "DealsWithGold" => SaleState::DealsWithGold,
            "DealsWithXboxGP" => SaleState::DealsWithXboxGP,
            "DealsWithPcGP" => SaleState::DealsWithPcGP,
            "DealsWithGPUltimate" => SaleState::DealsWithGPUltimate,
            "DealsWithEAPlay" => SaleState::DealsWithEAPlay,
            "NotOnSale" => SaleState::NotOnSale, 
            _ => panic!(),
        }
    }

    pub fn print(&self){
        match self.sale_state {
            SaleState::NotOnSale => {
                println!("     original_price {}  {}",self.original_price, self.currency);

            },
            SaleState::PublicSale => {
                println!("!!!! Sale {}% off, now for {}{} instead of {}{} available until {}",
                self.discount_ratio, self.sale_price, self.currency, self.original_price, self.currency, self.end_date);
            },
            SaleState::DealsWithGold => {
                println!("!!!! Sale With gold {}% off, now for {}{} instead of {}{} available until {}",
                self.discount_ratio, self.sale_price, self.currency, self.original_price, self.currency, self.end_date);
            },
            SaleState::DealsWithEAPlay => {
                println!("     save with EAPlay {}% off, now for {}{} instead of {}{} available until {}",
                self.discount_ratio, self.sale_price, self.currency, self.original_price, self.currency, self.end_date);
            },
            SaleState::DealsWithXboxGP => {
                println!("     save with Xbox GamePass {}% off, now for {}{} instead of {}{} available until {}",
                self.discount_ratio, self.sale_price, self.currency, self.original_price, self.currency, self.end_date);
            },
            SaleState::DealsWithPcGP => {
                println!("     save with PC GamePass {}% off, now for {}{} instead of {}{} available until {}",
                self.discount_ratio, self.sale_price, self.currency, self.original_price, self.currency, self.end_date);
            },
            SaleState::DealsWithGPUltimate => {
                println!("     save with GamePass Ultimate {}% off, now for {}{} instead of {}{} available until {}",
                self.discount_ratio, self.sale_price, self.currency, self.original_price, self.currency, self.end_date);
            },
        }

    }


    pub fn sale_state_string(&self) -> String{
        match self.sale_state {
            SaleState::NotOnSale => {
                String::from("NotOnSale")
            },
            SaleState::PublicSale => {
                String::from("PublicSale")
            },
            SaleState::DealsWithGold => {
                String::from("DealsWithGold")
            },
            SaleState::DealsWithEAPlay => {
                String::from("DealsWithEAPlay")
            },
            SaleState::DealsWithXboxGP => {
                String::from("DealsWithXboxGP")
            },
            SaleState::DealsWithPcGP => {
                String::from("DealsWithPcGP")
            },
            SaleState::DealsWithGPUltimate => {
                String::from("DealsWithGPUltimate")
            },
        }
    }

    pub fn new(availability: &availability::Availability) -> PurchaseAvailability {
        let mut result = PurchaseAvailability {
            id: "".to_string(),
            sale_state : SaleState::NotOnSale,
            original_price : 0.0,
            sale_price : 0.0,
            discount_ratio : 0,
            currency : String::from("USD"),
            start_date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc),
            end_date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc),
        };

        if let Some(condition) = &availability.conditions{
            result.start_date = condition.start_date.unwrap();
            result.end_date = condition.end_date.unwrap();
        }
        if let Some(id) = &availability.availability_id{
            result.id = String::from(id);
        }

        if let Some(order_managment) = &availability.order_management_data{
            let price = order_managment.price.list_price;
            let original_price = order_managment.price.m_s_r_p;

            result.currency = order_managment.price.currency_code.clone();
            result.original_price = original_price;
            result.sale_price = price;
            result.discount_ratio = PurchaseAvailability::get_sale_ratio(price, original_price);
            if price < original_price 
            {
                if let Some(remediation_required) = availability.remediation_required
                {
                    if remediation_required {
                        if let Some(remediations) = &availability.remediations {
                            for remediation in remediations.iter() {
                                match remediation.remediation_id.as_str() {
                                    remediation_values::XBOX_GAME_PASS  => {
                                        result.sale_state = SaleState::DealsWithXboxGP;
                                    },
                                    remediation_values::XBOX_LIVE_GOLD => {
                                        result.sale_state = SaleState::DealsWithGold;
                                    },
                                    remediation_values::EA_PLAY => {
                                        result.sale_state = SaleState::DealsWithEAPlay;
                                    },
                                    remediation_values::GAME_PASS_ULTIMATE =>{
                                        result.sale_state = SaleState::DealsWithGPUltimate;
                                    },
                                    remediation_values::PC_GAME_PASS  => {
                                        result.sale_state = SaleState::DealsWithPcGP;
                                    },
                                    _ => {},
                                } 
                            }
                        }
                    } else {
                        result.sale_state = SaleState::PublicSale;
                    }
                }                             
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_sale_ratio(){
        assert_eq!(67, PurchaseAvailability::get_sale_ratio(19.79, 60.0));
        assert_eq!(75, PurchaseAvailability::get_sale_ratio(57.48, 229.95));
        assert_eq!(20, PurchaseAvailability::get_sale_ratio(6.39, 7.99));
        assert_eq!(0, PurchaseAvailability::get_sale_ratio(7.99, 7.99));
        assert_eq!(100, PurchaseAvailability::get_sale_ratio(0.0, 7.99));
    }
}