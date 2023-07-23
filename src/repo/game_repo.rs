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

use crate::repo::shared::{Repo};
use super::shared;
use mongodb::bson::{doc, Document};
use mongodb::{Collection, Database, IndexModel};
use mongodb::options::{IndexOptions};
use crate::repo::purchase_option_repo::PurchaseAvailabilityRepo;
use crate::repo::models::game_model::{FetchGame, GameModel};



pub struct  GameRepo{
    data_base_collection : Collection<Document>,
    collection_name : String,
    purchase_availability_repo: PurchaseAvailabilityRepo
}



impl GameRepo{
    pub async fn new(data_base : & Database, purchase_availability_repo: PurchaseAvailabilityRepo) -> GameRepo{
        let collection_name = String::from("game");
        let data_base_collection = data_base.collection(&collection_name);
        let index_model = IndexModel::builder()
            .keys(doc! { "expire_at": 1 })
            .options(IndexOptions::builder().expire_after(std::time::Duration::from_secs(0)).build())
            .build();

        let expire_index_result = data_base_collection.create_index(index_model, None).await.expect("Failed to create the index.");
        println!("index created with name {}", expire_index_result.index_name);
        GameRepo{
            data_base_collection,
            collection_name,
            purchase_availability_repo
        }
    }

    pub async fn fetch_by_id(&self, id: &str) -> Option<GameModel>{
        let query = doc! {"id": id};
        self.fetch_by_query(query).await
    }

    pub async fn search_by_name<'a>(&self, name:&str, language: & 'a str, markets: &Vec<& 'a str>)-> Vec<FetchGame<'a>>{

        let pattern = regex::escape(&name).replace(" ", ".*");

        let query = doc!{
            "descriptions.body.name": doc!{
                "$regex": pattern,
                "$options": "i"
            }
        };
        
        let mut vec = Vec::<FetchGame>::new();

        for game_model in self.fetch_many_by_query(query).await.into_iter(){
            vec.push(game_model.get_game(language, markets));
        };
        vec
    }

    pub async fn fetch_game<'a>(&self, id:&str , language: & 'a str, markets: &Vec<& 'a str>)-> FetchGame<'a>{
        let query = doc! {
            "id": id,
            "descriptions.language":language,
            "purchase_options.market": doc!{
                "$in": markets,
            }

        };
        let game_model = self.fetch_by_query(query).await;
        if let Some(game_model) = game_model  {
            return  game_model.get_game(language, markets);
        }else{
            FetchGame::ElementNotFound(id.to_string())
        }
    }
}



impl shared::Repo<GameModel> for GameRepo{
    fn get_data_base_collection(&self) -> & Collection<Document> {
        & self.data_base_collection
    }
    fn get_collection_name(&self) -> & str{
        & self.collection_name
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_game_entity_to_document(){

    }
}
