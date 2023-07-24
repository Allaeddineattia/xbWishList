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


use actix_web::{get, HttpResponse, web, Responder, Scope};
use super::dto;
use dto::input::{GetGameRequest, SearchGamesRequest};
use dto::output::GameResponse;
use std::sync::Arc;
use utoipa::{ToSchema, OpenApi, openapi};

use crate::game::service::GameService;

#[utoipa::path(
context_path = "/game",
params(
("info" = GetGameRequest, Query,),
),
responses(
(status = 200, description = "Hello from api 2", body = GameResponse)
)
)]
#[get("/info")]
pub(super) async fn get_game(info: web::Query<GetGameRequest>, data: web::Data<Arc<GameService>>) -> impl Responder {

    let game = data.get_game_info(&info.id,& info.language, &info.markets.split(",").collect()).await;
    if let Some(game) = game {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(
                dto::output::GameResponse::new(game)
            )
    }else {
        HttpResponse::BadRequest()
            .body("no game found")
    }

}


#[utoipa::path(
context_path = "/game",
params(
("info" = SearchGamesRequest, Query,),
),
responses(
(status = 200, description = "Hello from api 2", body = GameResponse)
)
)]
#[get("/search")]
pub(super) async fn search_game(info: web::Query<SearchGamesRequest>, data: web::Data<Arc<GameService>>) -> impl Responder {
    let vec: Vec<GameResponse> = data.search_by_name(&info.query,& info.language, info.markets.split(",").collect()).await.into_iter().map(
        GameResponse::new
    ).collect();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(
            vec
        )

}


#[utoipa::path(
context_path = "/game",
responses(
(status = 200, description = "LeavingSoonFromGamePass", body = GameResponse)
)
)]
#[get("/game_pass")]
pub(super) async fn get_game_pass_leaving_soon(data: web::Data<Arc<GameService>>) -> impl Responder
{
    let vec: Vec<GameResponse> = data.get_game_pass_leaving_soon(None).await.into_iter().map(
        GameResponse::new
    ).collect();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(
            vec
        )
}



pub fn get_web_service(c: web::Data<Arc<GameService>>) -> Scope<> {
    web::scope("/game").
        app_data(c.clone()).
        service(get_game).
        service(search_game).
        service(get_game_pass_leaving_soon)

}

#[derive(OpenApi)]
#[openapi(
paths(
get_game,
search_game,
get_game_pass_leaving_soon
),
components(
schemas(SearchGamesRequest, GetGameRequest, GameResponse, dto::output::PurchaseOptionResponse, dto::output::AvailabilityResponse )
),
tags(
()
),
modifiers()
)]
struct GameApiDoc;

pub fn get_open_api() -> openapi::OpenApi
{
    GameApiDoc::openapi()
}




