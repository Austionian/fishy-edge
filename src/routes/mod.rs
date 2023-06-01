mod everything;
mod fish;
mod fish_avg;
mod fish_avgs;
mod fishs;
mod health_check;
mod login;
mod min_and_max;
mod presign_s3;
mod recipe;
mod recipes;
mod search;
mod structs;
mod user;

pub use everything::*;
pub use fish::fish as fish_route;
pub use fish_avg::fish_avg as fish_avg_route;
pub use fish_avgs::*;
pub use fishs::fishs as fishs_route;
pub use health_check::*;
pub use login::{login, register};
pub use min_and_max::*;
pub use presign_s3::*;
pub use recipe::*;
pub use recipes::*;
pub use search::search;
pub use structs::{Fish, Recipe};
pub use user::*;
