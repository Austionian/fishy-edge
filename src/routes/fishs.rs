use crate::routes::Fish;
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FishQuery {
    lake: String,
}

const VALID_LAKES: [&str; 4] = ["Store", "Superior", "Huron", "Michigan"];

/// Returns a JSON of all fish for a given lake. If no lake is supplied,
/// a 400 Bad Request will be returned. If an invalid lake is supplied,
/// the 'store' fish will be returned.
///
/// # Example
///
/// `.../fishs?lake=Huron`
///
///```json
/// {
///     [
///       "id": "1fe5c906-d09d-11ed-afa1-0242ac120002",
///       "fish_id": "1fe5c906-d09d-11ed-afa1-0242ac120022",
///       "name": "Herring",
///       // ...
///     ],
///     ...
/// }
///```
///
#[tracing::instrument(name = "Retreving all fish data", skip(lake, db_pool))]
#[get("/fishs")]
pub async fn fishs(lake: web::Query<FishQuery>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let mut lake = lake.lake.as_str();
    if !VALID_LAKES.contains(&lake) {
        tracing::warn!("Invalid lake supplied. Falling back to Store.");
        lake = "Store";
    }
    match get_fish_data(lake, &db_pool).await {
        Ok(data) => {
            tracing::info!("All fish type data has been queried from the db.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_fish_data(lake: &str, db_pool: &PgPool) -> Result<Vec<Fish>, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT 
            fish.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            fish.pcb,
            fish.protein,
            fish.omega_3,
            fish.omega_3_ratio,
            fish.mercury
        FROM fish
        JOIN fish_type
        ON fish.fish_type_id=fish_type.id
        WHERE fish.lake=$1;
        "#,
        lake
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
