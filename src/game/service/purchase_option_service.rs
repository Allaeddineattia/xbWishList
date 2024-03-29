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


use crate::game::purchase_option::PurchaseAvailability;
use crate::game::xbox_api_client::input_dto::catalog_response;

pub struct PurchaseOptionService {
}

impl PurchaseOptionService{
    pub fn new() -> Self {
        PurchaseOptionService { }
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
