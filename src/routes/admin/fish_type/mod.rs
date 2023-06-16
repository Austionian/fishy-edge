mod create;
mod update;
mod update_image;

pub use create::{new_fish_type, NewFishType};
pub use update::{insert_recipes_fish_type, update_fish_type};
pub use update_image::update_fish_type_image;
