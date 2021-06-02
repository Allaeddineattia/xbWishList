use crate::client::input_dto::availability;
use chrono::{DateTime, Utc, NaiveDateTime};
mod remediation_values {
    pub const XBOX_GAME_PASS: &str      = "9SJCZDHW896G";
    pub const PC_GAME_PASS: &str        = "9SQ1C79LQTJJ";
    pub const GAME_PASS_ULTIMATE: &str  = "9Q2FPGL45CQN";
    pub const XBOX_LIVE_GOLD: &str      = "9ZH7BH6P9RM7";
    pub const EA_PLAY: &str             = "9N8KCDNKJJQ6";

}

pub enum SaleState{
    PublicSale,
    DealsWithGold,
    DealsWithXboxGP,
    DealsWithPcGP,
    DealsWithGPUltimate,
    DealsWithEAPlay,
    NotOnSale,
}

pub struct PurchaseOption{
    pub market: String,
    pub purchase_options: Vec<PurchaseAvailability>,
}

impl PurchaseOption{


    pub fn print(&self){
        println!("_______Purchase__Options________");
        println!("Market {}", self.market);
        for option in &self.purchase_options {
            option.print();
        }

    }
    pub fn new(market: &str, purchase_options: Vec<PurchaseAvailability>) -> Self {
        PurchaseOption { market: String::from(market), purchase_options }
    }
}

pub struct PurchaseAvailability {
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
                print!("***Normal***");
            },
            SaleState::PublicSale => {
                print!("***Public Sale***");
            },
            SaleState::DealsWithGold => {
                print!("***DealsWithGold***");
            },
            SaleState::DealsWithEAPlay => {
                print!("***DealsWithEAPlay***");
            },
            SaleState::DealsWithXboxGP => {
                print!("***DealsWithXboxGP***");
            },
            SaleState::DealsWithPcGP => {
                print!("***DealsWithPcGP***");
            },
            SaleState::DealsWithGPUltimate => {
                print!("***DealsWithGPUltimate***");
            },
        }
        println!("sale_ratio <{}%>, original_price <{}>, sale_price<{} {}>, end_date<{}>", self.discount_ratio, self.original_price, self.sale_price, self.currency, self.end_date);
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
            sale_state : SaleState::NotOnSale,
            original_price : 0.0,
            sale_price : 0.0,
            discount_ratio : 0,
            currency : String::from("USD"),
            start_date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
            end_date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        };

        if let Some(condition) = &availability.conditions{
            result.start_date = condition.start_date.unwrap();
            result.end_date = condition.end_date.unwrap();
        }

        if let Some(order_managment) = &availability.order_management_data{
            let price = order_managment.price.list_price;
            let original_price = order_managment.price.m_s_r_p;
            let currency = order_managment.price.currency_code.clone();
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