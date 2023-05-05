use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct MinMaxQuery {
    lake: String,
    attr: String,
    avg: bool,
}

#[derive(serde::Serialize)]
struct Fish {
    name: String,
    anishinaabe_name: Option<String>,
    protein: Option<f32>,
    pcb: Option<f32>,
    omega_3: Option<f32>,
    mercury: Option<f32>,
}

const VALID_LAKES: [&str; 4] = ["Store", "Superior", "Huron", "Michigan"];
const VALID_ATTRS: [&str; 4] = ["protein", "pcb", "mercury", "omega_3_ratio"];

/// Returns a json of the fish with the min and max value for a given lake and
/// attribute.
///
/// # Example
///
/// `.../min_and_max?lake=Huron&attr=protein`
///
///```json
/// {
///     min: {
///       "id": "1fe5c906-d09d-11ed-afa1-0242ac120002",
///       "fish_id": "1fe5c906-d09d-11ed-afa1-0242ac120022",
///       "name": "Herring",
///       // ...
///     },
///     max: {
///       // ...
///     }
/// }
///```
///
#[tracing::instrument(name = "Retrieving the min and max fish values", skip(query, db_pool))]
#[get("/min_and_max")]
pub async fn min_and_max(
    query: web::Query<MinMaxQuery>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let mut lake = query.lake.as_str();
    if !VALID_LAKES.contains(&lake) {
        tracing::warn!("Invalid lake supplied. Falling back to Store.");
        lake = "Store";
    }
    let mut attr = query.attr.as_str();
    if !VALID_ATTRS.contains(&attr) {
        tracing::warn!("Invalid attr supplied.");
        attr = "protein";
    }
    match get_min_and_max_data(lake, attr, query.avg, &db_pool).await {
        Ok(data) => {
            tracing::info!("Min and max data has been queried from the db.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_min_and_max_data(
    lake: &str,
    attr: &str,
    avg: bool,
    db_pool: &PgPool,
) -> Result<Vec<Fish>, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT 
            fish_type.name,
            fish_type.anishinaabe_name,
            fish.protein,
            fish.pcb,
            fish.omega_3,
            fish.mercury
        FROM fish 
        JOIN fish_type
        ON fish.fish_type_id=fish_type.id
        WHERE ($2=(
            SELECT 
                MAX($2)
            FROM fish 
            WHERE lake=$1
        ) AND fish.lake=$1)
        OR
        ($2=(
            SELECT
                MIN($2)
            FROM fish
            WHERE lake=$1
        ) AND fish.lake=$1);
        "#,
        lake,
        attr
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
