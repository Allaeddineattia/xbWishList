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
use mongodb::bson::{Bson,Document, doc};
use mongodb::{Collection};
use async_trait::async_trait;

pub trait MongoEntity{
    fn to_document(&self)-> Document;
    fn create_from_document(doc : &Document) -> Self;

}

pub trait UniqueEntity{
    fn get_unique_selector(&self) -> Document;
}

#[async_trait]
pub trait  Repo <T> where T: MongoEntity + UniqueEntity + Sync + Send{
    fn get_data_base_collection(&self) -> & Collection;
    fn get_collection_name(&self) -> & str;

    async fn save(&self, entity: & T){
        let option = self.fetch(entity).await;
        if let Some(document) = option{
            let res = self.get_data_base_collection().update_one(entity.get_unique_selector(), entity.to_document(), None).await;
            let id = res.unwrap().upserted_id;
            if let Some(bson) = id {
                if let Bson::ObjectId(id) = bson{
                    println!("element with selector \"{}\" updated into collection \"{}\" with object id \"{}\"",
                             &entity.get_unique_selector(), self.get_collection_name(),id )
                }
            };
            return;
        }else {
            let document = entity.to_document();
            self.save_doc( document).await;
        }


    }

    async fn save_doc(&self, doc: Document){
        let data_base_collection = self.get_data_base_collection();
        let insertion_result = data_base_collection.insert_one(doc.clone(), None).await;
        match insertion_result {
            Ok(result) => {
                match result.inserted_id {
                    Bson::ObjectId(id) =>{
                        println!("element id \"{}\" inserted into collection \"{}\" with object id\
                         \"{}\"", doc.get_str("id").unwrap() ,self.get_collection_name(),id )
                    }
                    _ => {}
                }
            },
            Err(err) => {
                eprintln!("error on inserting element {:#?} because of error {:#?}", doc, err)
            }
        }
    }

    async fn fetch(&self, element: &T) -> Option<T>{
        let query = element.get_unique_selector();
        self.fetch_by_query(query).await
    }

    async fn fetch_by_id(&self, id: &str) -> Option<T>{
        let query = doc! {"id": id};
        self.fetch_by_query(query).await
    }

    async fn fetch_by_query(&self, query: Document) -> Option<T>{
        let data_base_collection = self.get_data_base_collection();
        let query_result = data_base_collection.find_one(query,None).await;
        match query_result {
            Ok(option) => {
                if let Some(document) = option{
                    return Some(T::create_from_document(&document))
                }
                None
            },
            Err(_error) => None
        }
    }

    async fn get_document_by_query(&self, query: Document)-> Option<Document>{
        let data_base_collection = self.get_data_base_collection();
        let query_result = data_base_collection.find_one(query,None).await;
        match query_result {
            Ok(option) => {
                if let Some(document) = option{
                    return Some(document);
                }
                None
            },
            Err(_error) => None
        }
    }

}

