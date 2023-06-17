mod admin;
mod everything;
mod favorite;
mod fish;
mod fish_avg;
mod fish_avgs;
mod fish_type;
mod fish_types;
mod fishs;
mod health_check;
mod login;
mod min_and_max;
mod presign_s3;
mod recipe;
mod recipes;
mod search;
mod structs;
mod unfavorite;
mod user;

pub use admin::{
    delete_fish, delete_recipe, new_fish, new_fish_type, new_recipe, update_fish, update_fish_type,
    update_fish_type_image, update_recipe,
};
pub use everything::*;
pub use favorite::{favorite_fish, favorite_recipe, favorites};
pub use fish::{fish as fish_route, get_is_favorite};
pub use fish_avg::fish_avg as fish_avg_route;
pub use fish_avgs::*;
pub use fish_type::fish_type as fish_type_route;
pub use fish_types::fish_types as fish_types_route;
pub use fishs::fishs as fishs_route;
pub use health_check::*;
pub use login::{login, register};
pub use min_and_max::*;
pub use presign_s3::*;
pub use recipe::*;
pub use recipes::*;
pub use search::search;
pub use structs::{Fish, Recipe};
pub use unfavorite::{unfavorite_fish, unfavorite_recipe};
pub use user::*;
