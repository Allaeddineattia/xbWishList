
use std::env;
mod client;
pub use client::module::game;
use tokio::task;


async fn send_req() -> Result<(), Box<dyn std::error::Error>>{
    let task1 = task::spawn(game::send_request(vec![String::from("9MZ11KT5KLP6")]));
    let task2 = task::spawn(game::send_request(vec![String::from("9PH339L3Z99C")]));

    let resp1 = task1.await??;
    let resp2 = task2.await??;
    game::get_info_from_response(&resp1);
    game::get_info_from_response(&resp2);
    Ok(())
}


fn main() {
    //let ids : Vec<String> = env::args().collect();// String::from("9MZ11KT5KLP6"),String::from("9PH339L3Z99C")
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(send_req());
    //let result = game::send_request(ids).await?;
    //game::get_info_from_response(&result);
    //game::read_from_file();
    //Ok(())
}
