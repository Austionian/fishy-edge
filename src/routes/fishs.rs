use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct AllFishData {
    id: Uuid,
    fish_id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
    woodland_fish_image: Option<String>,
    s3_fish_image: Option<String>,
    s3_woodland_image: Option<String>,
}

const VALID_LAKES: [&str; 4] = ["Store", "Superior", "Huron", "Michigan"];

/// Returns a JSON of all fish for a given lake. If no lake is supplied,
/// will return the 'store' fish. If an invalid lake is supplied,
/// will return 400 Bad Request.
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
///       "anishinaabe_name": "Okewis",
///       "fish_image": "herring.png",
///       "woodland_fish_image": "woodlandherring.webp",
///       "s3_fish_image": "",
///       "s3_woodland_image": ""
///     ],
///     ...
/// }
///```
///
#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
pub async fn fishs(req: HttpRequest, db_pool: web::Data<PgPool>) -> HttpResponse {
    let lake = req.match_info().get("lake").unwrap_or("Store");
    if !VALID_LAKES.contains(&lake) {
        tracing::error!("Invalid lake supplied.");
        return HttpResponse::BadRequest().finish();
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
pub async fn get_fish_data(lake: &str, db_pool: &PgPool) -> Result<Vec<AllFishData>, sqlx::Error> {
    let data = sqlx::query_as!(
        AllFishData,
        r#"
        SELECT 
            fish_type.id,
            fish.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image
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
