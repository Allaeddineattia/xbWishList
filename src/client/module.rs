
pub mod game{
    use reqwest::StatusCode;
    use reqwest::Url;
    pub async fn send_request(ids: Vec<String>)-> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let ids : String = ids.join(",");
        let url = Url::parse_with_params("https://displaycatalog.mp.microsoft.com/v7.0/products",&[("languages","en-US")
        ,("market","US")
        ,("bigIds", &ids) //9MZ11KT5KLP6,9PH339L3Z99C
        ,("actionFilter","Browse")
        ,("fieldsTemplate","details")])?;
        println!("Path: {:?}",url);
        let resp: reqwest::Response = client
        .get(url)
        .header("MS-CV", "\"\"").header("content_type", "multipart/form-data").send()
        .await?;
        

        match resp.status() {
            StatusCode::OK => {
                println!("{}", resp.text().await?)
            }
            s => println!("error: {:?}", resp.text().await?),
        }
        Ok(())
    }
}