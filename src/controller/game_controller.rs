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

use actix_web::{get, HttpResponse, web, Responder};
use crate::service::game_service::GameService;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
struct Obj {
    name: String,
}


pub struct GameController{
    //game_service: Rc<GameService>
    pub tita: String,
}

impl GameController {
    pub fn new() -> Self{
        Self{
            tita: "hehi".to_string()
        }
    }



    pub async fn search_game(web::Path((query)): web::Path<(String)>, data: web::Data<GameController>) -> impl Responder {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(
                Obj{
                    name: data.tita.clone() + & query
                }
            )

    }
}



