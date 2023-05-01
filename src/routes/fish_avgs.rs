use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
struct Fish {
    pub fish_id: Uuid,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub mercury: Option<f64>,
    pub omega_3: Option<f64>,
    pub omega_3_ratio: Option<f64>,
    pub pcb: Option<f64>,
    pub protein: Option<f64>,
}

/// Returns a JSON with every fish type and their averages.
///
/// # Example
///
/// `.../all_fish`
///
///```json
/// {
///     [
///       "fish_id": "1fe5c906-d09d-11ed-afa1-0242ac120022",
///       "name": "Herring",
///       // ...
///     ],
///     ...
/// }
///```
///
#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
#[get("/fish_avgs")]
pub async fn fish_avgs(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_all_fish_data(&db_pool).await {
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
async fn get_all_fish_data(db_pool: &PgPool) -> Result<Vec<Fish>, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT
            fish_type.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            AVG(pcb) as pcb,
            AVG(protein) as protein,
            AVG(mercury) as mercury,
            AVG(omega_3_ratio) as omega_3_ratio,
            AVG(omega_3) as omega_3
        FROM fish 
        JOIN fish_type ON fish.fish_type_id=fish_type.id
        GROUP BY fish_type.id;
        "#,
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
