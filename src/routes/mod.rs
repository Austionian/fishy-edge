use uuid::Uuid;

mod admin;
pub mod everything;
mod favorite;
mod get_fish;
mod get_fish_avg;
mod get_fish_avgs;
mod get_fishs;
mod health_check;
mod login;
pub mod min_and_max;
pub mod presign_s3;
pub mod recipe;
pub mod recipes;
mod search;
mod unfavorite;
mod user;

pub use admin::{
    create_fish_type, delete_fish, delete_recipe, new_fish, new_recipe, read_all_fish_types,
    read_fish_type, update_fish, update_fish_type, update_fish_type_image, update_recipe,
};
pub use everything::*;
pub use favorite::{favorite_fish, favorite_recipe, favorites};
pub use get_fish::{fish, get_is_favorite, FishResponse};
pub use get_fish_avg::fish_avg;
pub use get_fish_avgs::fish_avgs;
pub use get_fishs::fishs;
pub use health_check::*;
pub use login::{login, register};
pub use min_and_max::*;
pub use presign_s3::*;
pub use recipe::*;
pub use recipes::*;
pub use search::{search, SearchResult};
pub use unfavorite::{unfavorite_fish, unfavorite_recipe};
pub use user::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fish {
    pub fish_id: Uuid,
    pub fish_type_id: Uuid,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub mercury: Option<f32>,
    pub omega_3: Option<f32>,
    pub omega_3_ratio: Option<f32>,
    pub pcb: Option<f32>,
    pub protein: Option<f32>,
    pub lake: String,
    pub about: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub ingredients: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FishType {
    pub id: Uuid,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub about: String,
}
