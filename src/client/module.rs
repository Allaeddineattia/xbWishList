
pub mod game{
    use reqwest::StatusCode;
    use reqwest::Url;
    use crate::client::catalog_response;
    use crate::client::result;
    use std::fs;
    use anyhow::Result;
    use anyhow::anyhow;
    pub async fn send_request(ids: Vec<String>)-> Result<catalog_response::Response> {
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


        return match resp.status() {
            StatusCode::OK => {
                let result_test: catalog_response::Response = resp.json().await?;
                Ok(result_test)
                //get_info_from_response(&result_test);
            }
            s => {
                println!("error: {:?}", resp.text().await?);
                Err(anyhow!("error"))
            },
        }

    }

    pub fn get_info_from_response( result: &catalog_response::Response){
        for product in result.products.iter(){
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