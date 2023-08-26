mod analytics;
mod fish;
mod fish_type;
mod recipe;

pub use analytics::get_analytics;
pub use fish::{delete_fish, new_fish, update_fish};
pub use fish_type::{
    create_fish_type, read_all_fish_types, read_fish_type, update_fish_type, update_fish_type_image,
};
pub use recipe::{delete_recipe, new_recipe, update_recipe};
