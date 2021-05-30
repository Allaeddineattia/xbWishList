use crate::client::input_dto::catalog_response;
use crate::core::game;
use crate::repo::shared::MongoEntity;
use crate::repo::shared::Repo;
use mongodb::Database;
use crate::repo::game_repo::GameRepo;
/*struct Service {

}*/

pub async fn get_info_from_response( result: &catalog_response::Response, db : & Database) -> anyhow::Result<()>
{
    for product in result.products.iter(){
        let result: game::Game = game::Game::new(product);
        let id = result.id.clone();
        let game_repo = GameRepo::new(db);
        let doc = result.to_document();
        game_repo.save_doc(doc).await;
        let result = game_repo.get_document_by_id(&id).await;
        println!("id {}", id);
        let result = result.unwrap();
        let result = game::Game::create_from_document(&result);
        result.print();
    }
    Ok(())
}