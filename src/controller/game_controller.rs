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

use actix_web::{get, HttpResponse, web, Responder, Scope, HttpRequest, error};
use crate::service::game_service::GameService;
use std::rc::Rc;
use super::dto;
use actix_web::client::JsonPayloadError;
use std::error::Error;
use actix_web::error::InternalError;
use std::sync::Arc;

pub struct GameController{
    game_service: Arc<GameService>
}

impl GameController {

    pub fn new(game_service: Arc<GameService>) -> Self {
        GameController { game_service }
    }


    pub async fn search_game(web::Path((query)): web::Path<(String)>, data: web::Data<GameController>) -> impl Responder {
        let vec: Vec<super::dto::output::SearchResult> = data.game_service.search_game(&query,"US").await.into_iter().map(
            super::dto::output::SearchResult::new
        ).collect();
        HttpResponse::Ok()
            .content_type("application/json")
            .json(
                vec
            )

    }



    pub async fn get_game(form: web::Json<dto::input::GetGameInfo>, data: web::Data<GameController>) -> impl Responder {

        let game = data.game_service.get_game_info(&form.id, &form.language, form.markets.iter().map(|f|{&f[..]}).collect()).await;
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

    pub fn get_web_service(c: web::Data<Self>) -> Scope<> {
        web::scope("/game").
            app_data(c.clone()).
            route("/info", web::get().to(Self::get_game)).
            route("/search/{query}", web::get().to(Self::search_game))

    }

}



