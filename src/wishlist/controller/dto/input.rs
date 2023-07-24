
mod create_wishlist;
mod add_to_wishlist;
mod remove_from_wishlist;
mod update_wishlist_preference;


pub use create_wishlist::CreateWishlist;
pub use create_wishlist::WishlistElement;
pub use add_to_wishlist::AddToWishlistDTO;
pub use remove_from_wishlist::RemoveFromWishlistDTO;
pub use update_wishlist_preference::UpdateWishlistPreferenceDTO;
pub use update_wishlist_preference::UpdateWishlistElement;