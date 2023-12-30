mod create;
mod read;
mod read_all;
mod update;
mod update_image;

pub use create::create_fish_type;
pub use read::read_fish_type;
pub use read_all::read_all_fish_types;
pub use update::{insert_recipes_fish_type, update_fish_type};
pub use update_image::update_fish_type_image;
