
pub fn not_found_by_name(name: &str) -> String
{
    format!("wishlist with name {} is not found", name)
}

pub fn could_not_remove_item(name: &str, item_id: &str) -> String
{
    format!("removing item with id {} from wishlist with name {} failed", item_id, name)
}

pub fn redundant_game(id: &str) -> String
{
    format!("game with id {} is redundant, please make sure it's present only once", id)
}

pub fn game_not_found(id: &str) -> String
{
    format!("game with id {} is not found", id)
}
