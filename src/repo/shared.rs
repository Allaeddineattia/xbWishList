
use mongodb::bson::{Bson,Document, doc};
use mongodb::{Collection};
use async_trait::async_trait;

pub trait MongoEntity{
    fn to_document(&self)-> Document;
    fn create_from_document(doc : &Document) -> Self;
}

#[async_trait]
pub trait Repo{
    fn get_data_base_collection(&self) -> & Collection;

    fn get_collection_name(&self) -> & str;
    async fn  save_doc(&self, doc: Document){
        let data_base_collection = self.get_data_base_collection();
        let isertion_result = data_base_collection.insert_one(doc.clone(), None).await;
        match isertion_result{
            Ok(result) => {
                match result.inserted_id {
                    Bson::ObjectId(id) =>{
                        println!("element id \"{}\" inserted into collection \"{}\" with object id \"{}\"", doc.get_str("id").unwrap() ,self.get_collection_name(),id )
                    }
                    _ => {}
                }
            },
            Err(err) => {
                eprintln!("error on inserting element {:#?} because of error {:#?}", doc, err)
            }
        }
    }

    async fn get_document_by_id(&self, id: &str) -> Option<Document>{
        let data_base_collection = self.get_data_base_collection();
        let query = doc! {
            "id": id
        };
        let query_result = data_base_collection.find_one(query,None).await;
        match query_result {
            Ok(option) => option,
            Err(_error) => None
        }
    }
}