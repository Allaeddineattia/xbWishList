// use hyper::Client;
// use hyper::{Request, Body};
// use hyper::Uri;

// // A simple type alias so as to DRY.
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     // Still inside `async fn main`...
//     let mut uri = Uri::from_static("https://displaycatalog.mp.microsoft.com/v7.0/products?languages=en-US&market=US&bigIds=9MZ11KT5KLP6,9PH339L3Z99C&actionFilter=Browse&fieldsTemplate=details");
//     let path_query: & hyper::http::uri::PathAndQuery = uri.path_and_query().unwrap();
//     let req = Request::builder().uri(uri).header("MS-CV", "").body(Body::from(r#"{"library":"hyper"}"#))?;
//     let client = Client::new();


    
//     println!("{:?}", req);

//     // Await the response...
//     let resp = client.request(req).await?;

//     println!("Response: {}", resp.status());
//     Ok(())

// }
use std::collections::HashMap;
use reqwest::StatusCode;
mod client;
pub use client::module::game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vecr = vec![String::from("9MZ11KT5KLP6"),String::from("9PH339L3Z99C")];
    game::send_request(vecr).await
}
