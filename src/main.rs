
use std::env;
mod client;
pub use client::module::game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ids : Vec<String> = env::args().collect();// String::from("9MZ11KT5KLP6"),String::from("9PH339L3Z99C")
    //game::send_request(ids).await
    game::read_from_file();
    Ok(())
}
