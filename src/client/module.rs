
pub mod game{
    use reqwest::StatusCode;
    use reqwest::Url;
    use crate::client::catalog_response;
    use crate::client::result;
    use std::fs;
    pub async fn send_request(ids: Vec<String>)-> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let ids : String = ids.join(",");
        let url = Url::parse_with_params("https://displaycatalog.mp.microsoft.com/v7.0/products",&[("languages","en-US")
        ,("market","US")
        ,("bigIds", &ids) //9MZ11KT5KLP6,9PH339L3Z99C
        ,("actionFilter","Browse")
        ,("fieldsTemplate","details")])?;
        //println!("Path: {:?}",url);
        let resp: reqwest::Response = client
        .get(url)
        .header("MS-CV", "\"\"").header("content_type", "multipart/form-data").send()
        .await?;
        

        match resp.status() {
            StatusCode::OK => {
                let result_test: catalog_response::Response = resp.json().await?;
                get_info_from_response(&result_test);
            }
            s => println!("error: {:?}", resp.text().await?),
        }
        Ok(())
    }

    fn get_info_from_response( result: &catalog_response::Response){
        for product in result.products.iter(){
            // println!("product id {}", product.product_id);
            // for localized_properties in product.localized_properties.iter(){
            
            //     if let  Some(name) = &localized_properties.developer_name{
            //         println!("{} Made by {}",localized_properties.product_title,name)
            //     }
            //     for image in localized_properties.images.iter(){
            //         if image.image_purpose == "Poster" {
            //             let uri = String::from("http:") + &image.uri;
            //             println!("visit this uri for poster: {}", uri)
            //         }
            //     }
            //     if let Some(videos) = &localized_properties.videos{
            //         for video in videos.iter(){
            //             if video.video_purpose == "trailer"{
            //                 let uri = &video.uri;
            //                 println!("visit this uri for video trailer xml file: {}", uri)
            //             }
            //         }
            //     }
            

            //}
            // if let Some(product_a_schema) = &product.product_a_schema{
            //     println!("{}",product_a_schema)
            // }
            // if let Some(product_b_schema) = &product.product_b_schema{
            //     println!("{}",product_b_schema)
            // }
            let result: result::Result = result::Result::new(product);
            result.print();
            result::Result::print_price(product);
        }
    }



    pub fn read_from_file(){
        let contents = fs::read_to_string("./input")
            .expect("Something went wrong reading the file");
        let v: catalog_response::Response = serde_json::from_str(&contents).unwrap();
        get_info_from_response(&v);
    }

}