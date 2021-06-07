
pub mod microsoft_api{
    use reqwest::StatusCode;
    use reqwest::Url;
    use crate::client::input_dto::catalog_response;
    use anyhow::Result;
    use anyhow::anyhow;



    pub struct XboxLiveLanguage{
        name: String, //"Argentina"
        short_id: String, //"AR"
        identifier: String, //"es_AR"
        local: String, //"es-AR"
    }

    impl XboxLiveLanguage{
        fn new(name: &str , short_id: &str, identifier: &str, local: &str ) -> Self{
            XboxLiveLanguage{
                name: String::from(name),
                short_id: String::from(short_id),
                identifier: String::from(identifier),
                local: String::from(local),
            }
        }

        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn short_id(&self) -> &str {
            &self.short_id
        }
        pub fn identifier(&self) -> &str {
            &self.identifier
        }
        pub fn local(&self) -> &str {
            &self.local
        }

        pub fn argentina() -> Self{ XboxLiveLanguage::new("Argentina", "AR", "es_AR", "es-AR") }
        pub fn australia() -> Self{ XboxLiveLanguage::new("Australia", "AU", "en_AU", "en-AU") }
        pub fn austria() -> Self{XboxLiveLanguage::new("Austria", "AT", "de_AT", "de-AT")}
        pub fn belgium() -> Self{XboxLiveLanguage::new("Belgium", "BE", "fr_BE", "fr-BE")}
        pub fn belgium_nl() -> Self{XboxLiveLanguage::new("Belgium (NL)", "NL", "nl_BE", "nl-BE")}
        pub fn brazil() -> Self{XboxLiveLanguage::new("Brazil", "BR", "pt_BR", "pt-BR")}
        pub fn canada() -> Self{XboxLiveLanguage::new("Canada", "CA", "en_CA", "en-CA")}
        pub fn canada_fr() -> Self{XboxLiveLanguage::new("Canada (FR)", "CA", "fr_CA", "fr-CA")}
        pub fn czech_republic() -> Self{XboxLiveLanguage::new("Czech Republic", "CZ", "en_CZ", "en-CZ")}
        pub fn denmark() -> Self{XboxLiveLanguage::new("Denmark", "DK", "da_DK", "da-DK")}
        pub fn finland() -> Self{XboxLiveLanguage::new("Finland", "FI", "fi_FI", "fi-FI")}
        pub fn france() -> Self{XboxLiveLanguage::new("France", "FR", "fr_FR", "fr-FR")}
        pub fn germany() -> Self{XboxLiveLanguage::new("Germany", "DE", "de_DE", "de-DE")}
        pub fn greece() -> Self{XboxLiveLanguage::new("Greece", "GR", "en_GR", "en-GR")}
        pub fn hong_kong() -> Self{XboxLiveLanguage::new("Hong Kong", "HK", "en_HK", "en-HK")}
        pub fn hungary() -> Self{XboxLiveLanguage::new("Hungary", "HU", "en_HU", "en-HU")}
        pub fn india() -> Self{XboxLiveLanguage::new("India", "IN", "en_IN", "en-IN")}
        pub fn great_britain() -> Self{XboxLiveLanguage::new("Great Britain", "GB", "en_GB", "en-GB")}
        pub fn italy() -> Self{XboxLiveLanguage::new("Italy", "IT", "it_IT", "it-IT")}
        pub fn japan() -> Self{XboxLiveLanguage::new("Japan", "JP", "ja_JP", "ja-JP")}
        pub fn mexico() -> Self{XboxLiveLanguage::new("Mexico", "MX", "es_MX", "es-MX")}
        pub fn chile() -> Self{XboxLiveLanguage::new("Chile", "CL", "es_CL", "es-CL")}
        pub fn colombia() -> Self{XboxLiveLanguage::new("Colombia", "CO", "es_CO", "es-CO")}
        pub fn netherlands() -> Self{XboxLiveLanguage::new("Netherlands", "NL", "nl_NL", "nl-NL")}
        pub fn new_zealand() -> Self{XboxLiveLanguage::new("New Zealand", "NZ", "en_NZ", "en-NZ")}
        pub fn norway() -> Self{XboxLiveLanguage::new("Norway", "NO", "nb_NO", "nb-NO")}
        pub fn poland() -> Self{XboxLiveLanguage::new("Poland", "PL", "pl_PL", "pl-PL")}
        pub fn portugal() -> Self{XboxLiveLanguage::new("Portugal", "PT", "pt_PT", "pt-PT")}
        pub fn russia() -> Self{XboxLiveLanguage::new("Russia", "RU", "ru_RU", "ru-RU")}
        pub fn saudi_arabia() -> Self{XboxLiveLanguage::new("Saudi Arabia", "SA", "en_SA", "en-SA")}
        pub fn singapore() -> Self{XboxLiveLanguage::new("Singapore", "SG", "en_SG", "en-SG")}
        pub fn slovakia() -> Self{XboxLiveLanguage::new("Slovakia", "SK", "en_SK", "en-SK")}
        pub fn south_africa() -> Self{XboxLiveLanguage::new("South Africa", "ZA", "en_ZA", "en-ZA")}
        pub fn korea() -> Self{XboxLiveLanguage::new("Korea", "KR", "ko_KR", "ko-KR")}
        pub fn spain() -> Self{XboxLiveLanguage::new("Spain", "ES", "es_ES", "es-ES")}
        pub fn switzerland() -> Self{XboxLiveLanguage::new("Switzerland", "CH", "de_CH", "de-CH")}
        pub fn switzerland_fr() -> Self{XboxLiveLanguage::new("Switzerland (FR)", "CH", "fr_CH", "fr-CH")}
        pub fn united_arab_emirates() -> Self{XboxLiveLanguage::new("United Arab Emirates", "AE", "en_AE", "en-AE")}
        pub fn united_states() -> Self{XboxLiveLanguage::new("United States", "US", "en_US", "en-US")}
        pub fn ireland() -> Self{XboxLiveLanguage::new("Ireland", "IE", "en_IE", "en-IE")}
    }




    pub struct MicrosoftApiService{

    }

    impl MicrosoftApiService{
        pub fn new() -> Self{
            MicrosoftApiService{}
        }

        pub async fn get_games(ids: Vec<String>, language: XboxLiveLanguage)-> Result<catalog_response::Response> {
            println!("getting games for {:#?}", ids);

            let client = reqwest::Client::new();
            let ids : String = ids.join(",");
            let url = Url::parse_with_params("https://displaycatalog.mp.microsoft.com/v7.0/products",
                                             &[
                                                 ("languages", language.local()),
                                                 ("market", language.short_id()),
                                                 ("bigIds", &ids), //9MZ11KT5KLP6,9PH339L3Z99C
                                                 ("actionFilter","Browse"),
                                                 ("fieldsTemplate","details"),
                                             ])?;
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
                _ => {
                    println!("error: {:?}", resp.text().await?);
                    Err(anyhow!("error"))
                },
            }

        }
    }







    /*
    pub fn read_from_file(){
        let contents = fs::read_to_string("./input")
            .expect("Something went wrong reading the file");
        let v: catalog_response::Response = serde_json::from_str(&contents).unwrap();
        get_info_from_response(&v);
    }
    */
    

}