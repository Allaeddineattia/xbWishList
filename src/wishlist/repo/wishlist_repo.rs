
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
use mongodb::bson::{doc, Document};

use std::collections::{HashMap};
use mongodb::{Collection, Database};
use std::sync::Arc;
use crate::game::GameService;
use crate::shared::repository::Repo;
use crate::wishlist::models::wishlist_model::{WishlistElementModel, WishlistModel, WishlistPreferencesModel};
use crate::wishlist::wishlist::{Markets, Wishlist, WishlistElement, WishlistPreferences};


pub struct  WishlistRepo{
    data_base_collection : Collection<Document>,
    collection_name : String,
    game_service: Arc<GameService>

}

impl Repo<WishlistModel> for WishlistRepo{
    fn get_data_base_collection(&self) -> &Collection<Document> {
        &self.data_base_collection
    }
    fn get_collection_name(&self) -> &str {
        &self.collection_name
    }
}


impl WishlistRepo {
    pub fn new(data_base : & Database, game_service: Arc<GameService>) -> Self{
        let collection_name = String::from("wishlist");
        let data_base_collection = data_base.collection(&collection_name);
        Self{
            collection_name,
            data_base_collection,
            game_service
        }
    }

    async fn convert_model_to_entity(&self, model: WishlistModel) -> Wishlist{
        let preference = self.convert_model_wishlist_preferences(model.preference);
        let mut wishlist_elements = HashMap::<String, WishlistElement>::new();
        for wishlist_element_model in model.games.into_iter(){
            let wishlist_element = self.convert_model_to_wishlist_element(wishlist_element_model, &preference).await;
            if let Some(wishlist_element) = wishlist_element{
                wishlist_elements.insert(wishlist_element.game.id().to_string(),wishlist_element);
            }else {
                // make sure to treat when a game is missing
            }
        }
        Wishlist::new(
            model.name,
            model.owner_id,
            preference,
            wishlist_elements)
    }

    fn convert_model_wishlist_preferences(&self, model: WishlistPreferencesModel) -> WishlistPreferences{
        WishlistPreferences::new(model.language,  Markets::from_vec_str(model.markets).0)
    }
    async fn convert_model_to_wishlist_element(&self, model: WishlistElementModel, pref: &WishlistPreferences)-> Option<WishlistElement>{
        let game_id = &model.game_id;
        let language = &pref.language;
        let markets: Markets = Markets::from_vec_str(model.markets).0;

        if let Some(game ) = self.game_service.get_game_info(game_id, language, &markets.to_vec()).await
        {
            Some(WishlistElement::new(game, markets))
        }
        else
        {
            None
        }
    }

    pub async fn fetch_by_name(&self, name: &str, owner_id: &str) -> Option<Wishlist>{
        let query = doc! {"name": name, "owner_id": owner_id};
        let fetch:Option<WishlistModel> = self.fetch_one_by_query(query).await;
        if let Some(model) = fetch{
            Some(self.convert_model_to_entity(model).await)
        }else{
            None
        }
    }

    pub async fn delete_by_name(&self, name: &str, owner_id: &str) -> bool{
        let query = doc! {"name": name, "owner_id": owner_id};
        let result = self.get_data_base_collection().delete_one(query, None).await;
        match result {
            Ok(res) => {
                println!("deleted {} ", res.deleted_count); 
                true
            },
            Err(error) => {
                println!("delete error {}", error);
                false
            }
        }
    }

    pub async fn get_all(&self, owner_id: &str) -> Vec<Wishlist>{
        let mut vec = Vec::<Wishlist>::new();
        let query = doc! {"owner_id": owner_id};
        for model in self.fetch_many_by_query(query).await.into_iter(){
            vec.push(self.convert_model_to_entity(model).await);
        };
        vec
    }

    pub async fn save_wishlist(&self, wishlist: &Wishlist){
        self.save(&self.entity_to_model(wishlist)).await
    }

    fn entity_to_model(&self, wishlist: &Wishlist) -> WishlistModel{
        WishlistModel{
            name: wishlist.name.clone(),
            owner_id: wishlist.owner_id.clone(),
            games: wishlist.games.iter().map(|x|{self.element_entity_to_model(x.1)}).collect(),
            preference: self.preference_entity_to_model(wishlist.preference())
        }
    }

    fn preference_entity_to_model(&self, wishlist_preferences: &WishlistPreferences) -> WishlistPreferencesModel{
        WishlistPreferencesModel{
            language: wishlist_preferences.language.clone(),
            markets: wishlist_preferences.markets().to_vec().iter().map(|str|{str.to_string()}).collect()
        }
    }

    fn element_entity_to_model(&self, wishlist_element: &WishlistElement) -> WishlistElementModel{
        let markets_vec = wishlist_element.markets.to_vec();
        let mut markets = Vec::<String>::new();
        if !markets_vec.is_empty(){
            markets = markets_vec.iter().map(|str|{str.to_string()}).collect();
        };
        WishlistElementModel{
            game_id: wishlist_element.game.id().to_string(),
            markets
        }
    }





}



