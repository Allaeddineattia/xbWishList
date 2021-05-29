use crate::client::input_dto::catalog_response;
use crate::core::game;
use crate::repo::shared::mongo_entity;
use mongodb::Database;
struct Service {

}

pub async fn get_info_from_response( result: &catalog_response::Response, db : & Database) -> anyhow::Result<()>
{
    for product in result.products.iter(){
        let result: game::Game = game::Game::new(product);

        
        
        //result.print();
        let doc = result.to_entity();
        let collection = db.collection("game");
        let result = collection.insert_one(doc, None).await?;
        println!("result {:#?}",result);
        //result::Result::print_price(product);
    }
    Ok(())
}