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
use utoipa::{ToSchema};

#[derive(Serialize, Deserialize, Clone,  ToSchema)]
pub struct SearchGamesRequest {
    #[schema(example = "sekiro")]
    pub query: String,
    #[schema(example = "en-US")]
    pub language: String,
    #[schema(example = "US,FR")]
    pub markets: String

}

#[derive(Serialize, Deserialize, Clone,  ToSchema)]
pub struct GetGameRequest {
    #[schema(example = "9N9X9PC8V9CP")]
    pub id: String,
    #[schema(example = "en-US")]
    pub language: String,
    #[schema(example = "US,FR")]
    pub markets: String

}





