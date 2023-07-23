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


use serde::{Deserialize, Serialize};
use actix_web::{get, HttpResponse, web, Responder, Scope};
use crate::service::game_service::GameService;
use super::dto;
use std::sync::Arc;
use utoipa::{ToSchema, OpenApi, openapi};

#[derive(Serialize, Deserialize, Clone,  ToSchema)]
pub struct Info {
    #[schema(example = "sekiro")]
    query: String,
    #[schema(example = "en-US")]
    pub language: String,
    #[schema(example = "US,FR")]
    pub markets: String


}

#[derive(Serialize, Deserialize, Clone,  ToSchema)]
pub struct GetGameInfo{
    #[schema(example = 1)]
    pub id: String,
    #[schema(example = "en-US")]
    pub language: String,
    #[schema(example = "US,FR")]
    pub markets: String

}

#[utoipa::path(
context_path = "/game",
params(
("info" = GetGameInfo, Query,),
),
responses(
(status = 200, description = "Hello from api 2", body = Info)
)
)]
#[get("/info")]
pub(super) async fn get_game(info: web::Query<GetGameInfo>, data: web::Data<Arc<GameService>>) -> impl Responder {

    let game = data.get_game_info(&info.id,& info.language, &info.markets.split(",").collect()).await;
    if let Some(game) = game {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(
                dto::output::GameInfo::new(game)
            )
    }else {
        HttpResponse::BadRequest()
            .body("no game found")
    }

}


#[utoipa::path(
context_path = "/game",
params(
("info" = Info, Query,),
),
responses(
(status = 200, description = "Hello from api 2", body = Info)
)
)]
#[get("/search")]
pub(super) async fn search_game(info: web::Query<Info>,data: web::Data<Arc<GameService>>) -> impl Responder {
    let vec: Vec<dto::output::GameInfo> = data.search_by_name(&info.query,& info.language, info.markets.split(",").collect()).await.into_iter().map(
        dto::output::GameInfo::new
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
(status = 200, description = "LeavingSoonFromGamePass", body = Info)
)
)]
#[get("/game_pass")]
pub(super) async fn get_game_pass_leaving_soon(data: web::Data<Arc<GameService>>) -> impl Responder
{
    let vec: Vec<dto::output::GameInfo> = data.get_game_pass_leaving_soon(None).await.into_iter().map(
        dto::output::GameInfo::new
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
schemas(Info, GetGameInfo )
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




