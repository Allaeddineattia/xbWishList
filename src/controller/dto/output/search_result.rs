use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult{
    pub height: u32, //1080
    pub width: u32, //1080
    pub image_type: String, // "BoxArt"
    pub icon_u_r_l: String,//url
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub store_u_r_l: String,
}

impl SearchResult{
    fn get_store_u_r_l(id: &str, language: &str, title: &str) -> String {
        String::from("https://www.microsoft.com/") + language + "/p/" +
            &*title.trim().replace(" ", "-").replace(":", "").replace("'", "")
                .replace("|", "").replace("&", "").to_lowercase() + "/"
            + id
    }
    pub fn new(product : crate::client::input_dto::search_response::SearchItemProduct) -> Self{
        let store_u_r_l = Self::get_store_u_r_l(&product.product_id, "en-US", &product.title);
        Self{
            height: product.height,
            width: product.width,
            image_type: product.image_type,
            icon_u_r_l: product.icon,
            id: product.product_id,
            r#type: product.r#type,
            title: product.title,
            store_u_r_l
        }
    }
}
