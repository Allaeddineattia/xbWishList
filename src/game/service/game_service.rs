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



use std::sync::Arc;
use crate::game::game::Game;
use crate::game::helpers::uri::get_uri;
use crate::game::service::PurchaseOptionService;
use crate::game::xbox_api_client;
use xbox_api_client::MicrosoftApiClient;
use xbox_api_client::input_dto::catalog_response;
use xbox_api_client::input_dto::leaving_soon_response::LeavingSoonResponse;
use xbox_api_client::input_dto::search_response::{SearchItem, SearchItemProduct, SearchResponse};
use xbox_api_client::markets;
use xbox_api_client::markets::{MARKETS, XboxLiveLanguage};
use crate::game::repo::game_model::{FetchGame, GameModel};
use crate::game::repo::game_repo::GameRepo;
use crate::shared::repository::Repo;

pub struct GameService {
    purchase_option_service: Arc<PurchaseOptionService>,
    game_repo: Arc<GameRepo>,
    client : Arc<MicrosoftApiClient>
}

impl GameService{

    pub fn new(game_repo: Arc<GameRepo>) -> Self {
        let purchase_option_service = Arc::new(PurchaseOptionService::new());
        let client = Arc::new(MicrosoftApiClient::new());
        GameService {
            purchase_option_service, 
            game_repo,
            client
        }
    }


    fn abstract_product_to_game(&self, product: &catalog_response::Product, language: & str, market: &  XboxLiveLanguage::<'static>) -> Game{
        let mut game = Game::from_product(product, language);
        let store_uri = get_uri(&game.name(), market.local(), &game.id());
        let sales = self.purchase_option_service.get_sales(product);
        game.add_purchase_option(market.short_id(), store_uri, sales);
        game
    }

    pub fn abstract_result_to_games(&self, result: &catalog_response::Response, language: & str, market: &  XboxLiveLanguage::<'static>) -> Vec<Game>{
        result.products.iter().map(|product|{
            self.abstract_product_to_game(product, language, market)
        }).collect()
    }

    pub async fn save_game(&self, game:&Game){
        let mut game_entity: GameModel;
        println!("fetching result game id {}", game.id());
        let fetch_result = self.game_repo.fetch_by_id(game.id()).await;

        if let Some(entity) = fetch_result{
            println!("result fetched entity id {}", &entity.id);
            game_entity = entity;
        }else{
            game_entity = GameModel::new(game.id());
        }
        game_entity.add_info(game).expect("TODO: panic message");

        self.game_repo.save(&game_entity).await;
    }

    pub async fn save_response(&self, result: &catalog_response::Response, language: & str, market: &  XboxLiveLanguage::<'static>) -> anyhow::Result<()>
    {

        for product in result.products.iter(){
            println!("saving result with product with id {}", product.product_id);
            self.save_game(&(self.abstract_product_to_game(product, language, market))).await;
        }
        Ok(())
    }

    async fn cure_description_missing (&self, id: &str, language: &str){
        println!("game description is missing trying to cure the problem");
        let market = & markets::UNITED_STATES;
        let result = self.client.get_games(vec![id.to_string()], language, market.short_id()).await;
        if let Ok(result) = result{
            self.save_response(&result, language, &market).await.expect("TODO: panic message");
        }

    }

    async fn cure_markets_missing<'a>(&'a self, id: &str, markets: &Vec<&str>){
        println!("game description is missing markets to cure the problem");
        for market in markets{
            let market = MARKETS.get(market);
            if let Some(market) = market{
                self.save_result(&id, market).await;
            }
        }

    }

    async fn save_result(&self, id: &str, market: &  XboxLiveLanguage::<'static>)
    {
        if let Ok(result) = self.client.get_games(vec![id.to_string()], market.local(), market.short_id()).await{

            self.save_response(&result, market.local(), market).await.expect("TODO: panic message");

        }
    }

    pub async fn get_game_pass_leaving_soon(&self, language: Option<& str> )-> Vec<Game>
    {
        let language = language.unwrap_or_else(|| { "en-US" });
        let markets = vec!["US"];
        let response: Vec<LeavingSoonResponse> = self.client.get_game_pass_leaving_soon().await.unwrap();
        let mut result: Vec<Game> = vec![];
        for game in response
        {
            if let Some(id) = game.id
            {
                if let Some(game_info) = self.get_game_info(&id, language, &markets).await
                {
                    result.push(game_info);
                }
            }
        };
        return result;

    }

    pub async fn get_game_info(&self, id:&str , language: & str, markets: &Vec<& str>) -> Option<Game>{
        let id = &id.to_uppercase()[..];
        let fetched_game = self.game_repo.fetch_game(id, language, &markets).await;
        if let FetchGame::Fetched(game) = fetched_game
        {
            println!("\n-----------------\n");
            game.print_price();
            return Some(game);
        };
        self.cure_description_missing (id, language).await;
        self.cure_markets_missing(id, &markets).await;
        self.get_game_if_exists(id, language, markets).await
        
    }

    async fn get_game_if_exists(&self, id:&str , language: & str, markets: &Vec<& str>) -> Option<Game>
    {
        if let FetchGame::Fetched(game) = self.game_repo.fetch_game(id, language, &markets).await
        {
            println!("\n-----------------\n");
            game.print_price();
            return Some(game);
        }
        None
    }

    /*
    async fn get_game_info_from_all_markets(&self, id: &str){
        for market in MARKETS.into_iter(){
            let market = market.1;
            self.save_result(id, market).await;
        }
    }
    */
    fn id_exists_in_games(id: &str, games: & Vec<Game>) -> bool{
        for game in games{
            if id == game.id() {
                return true
            }
        }
        false
    }

    pub async fn search_by_name(&self, query: &str, language: &str, markets: Vec<& str>)-> Vec<Game>{

        let mut games = self.fetch_games_by_name(query, language, &markets).await;

        let result = self.client.search_games(query, "en-US", "US").await;
        if let Ok(mut search_response) = result{
            let mut v = self.parse_search_response(& mut search_response, language, &markets).await;
            games.append(& mut v);
        };
        games
    }

    async fn fetch_games_by_name(&self, query: &str, language: &str, markets: &Vec<& str>)->Vec<Game>
    {
        let mut games = Vec::<Game>::new();
        let entities = self.game_repo.search_by_name(query, language, &markets).await;
        for fetched_game in entities{
            if let Some(game) = self.fetch_game_by_name(fetched_game, language, markets).await
            {
                games.push(game);
            }
        };
        games
    }

    async fn fetch_game_by_name<'a>(&self, fetched_game:  FetchGame<'a>, language: &str, markets: &Vec<& str>)-> Option<Game>{
        if let FetchGame::Fetched(game) = fetched_game
        {
            println!("\n-----------------\n");
            game.print_price();
            return Some(game);
        }
        else if let FetchGame::ElementNotFound(_) = fetched_game
        {
            return None;
        }
        else if let FetchGame::MissingDescription(id, missing_language) = fetched_game
        {
            self.cure_description_missing(&id, missing_language).await;
            return self.get_game_if_exists(&id, language, markets).await;
        }
        else if let FetchGame::MissingMarkets(id,missing_markets) = fetched_game
        {
            self.cure_markets_missing(&id, &missing_markets).await;
            return self.get_game_if_exists(&id, language, markets).await;
        }

        None
    }



    async fn parse_search_response(&self, search_response: & mut SearchResponse,  language: &str, markets: &Vec<& str>) -> Vec<Game>
    {
        let mut games = Vec::<Game>::new();
        let results = & mut search_response.results;
        for mut item in results.into_iter()
        {
            let mut v = self.parse_item(& mut item, language, markets).await;
            games.append(&mut v);
        };
        games
    }

    async fn parse_item(&self, item_product: &mut SearchItem, language: &str, markets: &Vec<& str>) -> Vec<Game>
    {
        let mut games = Vec::<Game>::new();
        let products = & mut item_product.products;
        for mut product in products.into_iter(){
            product.icon = format!("https:{}", &product.icon);
            println!("product found \nid: {} \ntitle: {} \nimage url: {}", product.product_id, product.title, product.icon);
            if ! Self::id_exists_in_games(&product.product_id,&games){
                let game = self.get_game_info(&product.product_id, language, &markets).await;
                if let Some(game) = game{
                    games.push(game);
                };
            }
        }
        games
    }


    pub async fn search_game(&self, query: &str, language: &str) -> Vec<SearchItemProduct> {
        let market = MARKETS.get(language);
        let mut vec = Vec::<SearchItemProduct>::new();
        if let Some(market) = market{
            let result = self.client.search_games(query, &market.local(), &market.short_id()).await;
            if let Ok(search_response) = result{
                for item in search_response.results.into_iter(){
                    for mut product in item.products.into_iter(){
                        product.icon = "https:".to_string() + &product.icon;
                        println!("product found \nid: {} \ntitle: {} \nimage url: {}", product.product_id, product.title, product.icon);
                        vec.push(product);
                    }
                }
            }
        } else {
            println!("language {} not supported", language);
        }
        vec
        
    }


}





