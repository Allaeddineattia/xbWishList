
pub mod game{
    use reqwest::StatusCode;
    use reqwest::Url;
    use crate::client::catalog_response;
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
                println!("{:?}", result_test.products[0].last_modified_date);
                //catalog_response::parse(&result_test);
                //let v: Value = serde_json::from_str(&resp.text().await?)?;
                //println!("{:?}", v["big_ids"]);
                //println!("{:?}", v["Products"])
            }
            s => println!("error: {:?}", resp.text().await?),
        }
        Ok(())
    }
}