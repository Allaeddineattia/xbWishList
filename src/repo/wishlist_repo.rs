
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
use crate::{core::wishlist, repo::shared::Repo};
use mongodb::bson::{doc, Document, Bson};
use super::shared;

use std::collections::{HashMap, HashSet};
use mongodb::{Collection, Database};
use crate::core::wishlist::{Wishlist, WishlistElement, Markets};
use std::sync::Arc;
use crate::repo::game_repo::GameRepo;
use crate::repo::models::wishlist_model;
use crate::core::game::Game;
use crate::repo::models::wishlist_model::{WishlistModel, WishlistElementModel, WishlistPreferencesModel};
use crate::repo::models::game_model::FetchGame;


pub struct  WishlistRepo{
    data_base_collection : Collection,
    collection_name : String,
    game_repo: Arc<GameRepo>

}

impl WishlistRepo {
    pub fn new(data_base : & Database, game_repo: Arc<GameRepo>) -> Self{
        let collection_name = String::from("wishlist");
        let data_base_collection = data_base.collection(&collection_name);
        Self{
            collection_name,
            data_base_collection,
            game_repo
        }
    }

    async fn convert_model_to_entity(&self, model: wishlist_model::WishlistModel) -> wishlist::Wishlist{
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
        wishlist::Wishlist::new(
            model.name,
            preference,
            wishlist_elements)
    }

    fn convert_model_wishlist_preferences(&self, model: wishlist_model::WishlistPreferencesModel) -> wishlist::WishlistPreferences{
        wishlist::WishlistPreferences::new(model.language,  wishlist::Markets::from_vec_str(model.markets).0)
    }
    async fn convert_model_to_wishlist_element(&self, model: wishlist_model::WishlistElementModel, pref: &wishlist::WishlistPreferences)-> Option<wishlist::WishlistElement>{
        let game_id = &model.game_id;
        let language = &pref.language;
        let markets: Markets = Markets::from_vec_str(model.markets).0;

        let fetch = self.game_repo.fetch_game(game_id, language, &markets.to_vec()).await;

        match fetch {
            FetchGame::Fetched(game) => {
                return Some(wishlist::WishlistElement::new(game, markets))
            }
            FetchGame::MissingMarkets(_) => {
                None
            }
            FetchGame::MissingDescription(_) => {
                None
            }
            FetchGame::ElementNotFound(_) => {
                None
            }
        }

    }

    pub async fn fetch_by_name(&self, name: &str) -> Option<wishlist::Wishlist>{
        let query = doc! {"name": name};
        let fetch:Option<wishlist_model::WishlistModel> = self.fetch_by_query(query).await;
        if let Some(model) = fetch{
            Some(self.convert_model_to_entity(model).await)
        }else{
            None
        }
    }

    pub async fn save_wishlist(&self, wishlist: &wishlist::Wishlist){
        self.save(&self.entity_to_model(wishlist)).await
    }

    fn entity_to_model(&self, wishlist: &wishlist::Wishlist) -> WishlistModel{
        WishlistModel{
            name: wishlist.name.clone(),
            games: wishlist.games.iter().map(|x|{self.element_entity_to_model(x.1)}).collect(),
            preference: self.preference_entity_to_model(wishlist.preference())
        }
    }

    fn preference_entity_to_model(&self, wishlist_preferences: &wishlist::WishlistPreferences) -> WishlistPreferencesModel{
        WishlistPreferencesModel{
            language: wishlist_preferences.language.clone(),
            markets: wishlist_preferences.markets().to_vec().iter().map(|str|{str.to_string()}).collect()
        }
    }

    fn element_entity_to_model(&self, wishlist_element: &wishlist::WishlistElement) -> WishlistElementModel{
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

impl Repo<WishlistModel> for WishlistRepo{
    fn get_data_base_collection(&self) -> &Collection {
        &self.data_base_collection
    }

    fn get_collection_name(&self) -> &str {
        &self.collection_name
    }
}


