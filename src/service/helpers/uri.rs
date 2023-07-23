
fn get_name_for_uri(name: &str ) -> String
{
    name.trim()
        .replace(" ", "-")
        .replace(":", "")
        .replace("'", "")
        .replace("|", "")
        .replace("&", "")
        .to_lowercase()
}

pub fn get_uri(name: &str, market: &str, product_id: &str) -> String
{
    format!(
        "https://www.xbox.com/{}/games/store/{}/{}",
        market,
        get_name_for_uri(name),
        product_id
    )
}