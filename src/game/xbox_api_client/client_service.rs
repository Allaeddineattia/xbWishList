/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(dead_code)]
use reqwest::StatusCode;
use reqwest::Url;
use anyhow::Result;
use anyhow::anyhow;
use super::input_dto::{catalog_response, leaving_soon_response, search_response};



pub struct MicrosoftApiClient {
    client: reqwest::Client,
}

impl MicrosoftApiClient {
    pub fn new() -> MicrosoftApiClient {
        MicrosoftApiClient {client: reqwest::Client::new()}
    }

    pub async fn get_games(&self, ids: Vec<String>, language: & str, market: & str)-> Result<catalog_response::Response> {
        println!("getting info for games with \"language\"<{}> \"market\"<{}> ids {:#?} ", language, market, ids);
        let ids : String = ids.join(",");
        let url = Url::parse_with_params("https://displaycatalog.mp.microsoft.com/v7.0/products",
                                         &[
                                             ("languages", language),
                                             ("market", market),
                                             ("bigIds", &ids), //9MZ11KT5KLP6,9PH339L3Z99C
                                             ("actionFilter","Browse"),
                                             ("fieldsTemplate","details"),
                                         ])?;
        println!("getting info {:?}",url);

        let resp: reqwest::Response = self.client
            .get(url)
            .header("MS-CV", "\"\"").header("content_type", "multipart/form-data").send()
            .await?;

        println!("getting info {:?}",resp.status());
        return match resp.status() {
            StatusCode::OK => {
                return match resp.json().await
                {
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(e) =>
                        {
                            eprintln!("hehi hehi gadek behi {}", e);
                            Err(anyhow::Error::from(e))
                        }
                }
            }
            _ => {
                eprintln!("error: {:?}", resp.text().await?);
                Err(anyhow!("error"))
            },
        }

    }

    /*
    fn get_games_on_deals(ids: Vec<String>, language: & 'static str, market: & 'static str){
        let xgpleavingsoonconsole = "https://catalog.gamepass.com/sigls/v2?id=393f05bf-e596-4ef6-9487-6d4fa0eab987&language=en-us&market=US";
        let onPublicSale = "https://reco-public.rec.mp.microsoft.com/channels/Reco/V8.0/Lists/Computed/Deal?Market=us&Language=en&ItemTypes=Game&deviceFamily=Windows.Xbox&count=2000&skipitems=100";// max 200
        let xpgAllConsoleGames = "https://catalog.gamepass.com/sigls/v2?id=f6f1f99f-9b49-4ccd-b3bf-4d9767a77f5e&language=en-us&market=US";
        let xpgCommingCOnsole = "https://catalog.gamepass.com/sigls/v2?id=095bda36-f5cd-43f2-9ee1-0a72f371fb96&language=en-us&market=US";
        let topPaid = "https://reco-public.rec.mp.microsoft.com/channels/Reco/V8.0/Lists/Computed/TopPaid?Market=us&Language=en&ItemTypes=Game&deviceFamily=Windows.Xbox&count=2000&skipitems=0";//200 by request


    }*/

    pub async fn get_game_pass_leaving_soon(&self) -> Result<Vec<leaving_soon_response::LeavingSoonResponse>>
    {
        let url = Url::parse("https://catalog.gamepass.com/sigls/v2?id=393f05bf-e596-4ef6-9487-6d4fa0eab987&language=en-us&market=US").unwrap();
        let resp: reqwest::Response = match self.client.get(url).send().await
        {
            Ok(response) => response,
            Err(e) => return Err(anyhow!(e)),
        };

        match resp.status(){
            StatusCode::OK => {}
            _ => {
                println!("error: {:?}", resp.text().await?);
                return Err(anyhow!("error"));
            },
        };
        return match resp.json().await
        {
            Ok(v) => {
                Ok(v)
            },
            Err(e) =>
                {
                    eprintln!("{}", e);
                    Err(anyhow::Error::from(e))
                }
        }



    }

    pub async fn search_games(&self, query: &str, language: & str, market: & str)-> Result<search_response::SearchResponse> {
        let url = Url::parse_with_params("https://displaycatalog.mp.microsoft.com/v7.0/productFamilies/autosuggest",
                                         &[
                                             ("languages", language),
                                             ("market", market),
                                             ("query", query), //9MZ11KT5KLP6,9PH339L3Z99C
                                             ("productFamilyNames","Games"),
                                         ])?;

        let resp: reqwest::Response = self.client
                                        .get(url)
                                        .header("MS-CV", "\"\"").header("content_type", "multipart/form-data").send()
                                        .await?;
        return match resp.status() {
            StatusCode::OK => {
                let result_test: search_response::SearchResponse = resp.json().await?;
                Ok(result_test)
            }
            _ => {
                println!("error: {:?}", resp.text().await?);
                Err(anyhow!("error"))
            },
        }
    }

}
    

