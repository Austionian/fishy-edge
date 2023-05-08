use actix_web::{get, web, HttpResponse};
use sqlx::{PgPool, Postgres};

#[derive(serde::Deserialize)]
pub struct MinMaxQuery {
    lake: Option<String>,
    attr: String,
}

#[derive(serde::Serialize, sqlx::FromRow, Clone)]
struct Fish {
    name: String,
    anishinaabe_name: Option<String>,
    value: Option<f32>,
}

#[derive(serde::Serialize)]
struct Data {
    min: Option<Fish>,
    max: Option<Fish>,
}

const VALID_LAKES: [&str; 4] = ["Store", "Superior", "Huron", "Michigan"];
const VALID_ATTRS: [&str; 4] = ["protein", "pcb", "mercury", "omega_3_ratio"];

/// Returns a json of the fish with the min and max value for a given lake and
/// attribute. If no lake specified, will return min and max values for all fish
/// averages.
///
/// # Example
///
/// `.../min_and_max?lake=Huron&attr=protein`
///
///```json
/// {
///     min {
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
    let attr = query.attr.as_str();
    if !VALID_ATTRS.contains(&attr) {
        tracing::warn!("Invalid attr supplied.");
        return HttpResponse::NotAcceptable().finish();
    }
    match &query.lake {
        Some(lake) => {
            let lake = lake.as_str();
            if !VALID_LAKES.contains(&lake) {
                tracing::warn!("Invalid lake supplied. Falling back to Store.");
                return HttpResponse::NotAcceptable().finish();
            }
            match get_min_and_max_data(lake, attr, &db_pool).await {
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
        None => match get_min_and_max_of_avg_data(attr, &db_pool).await {
            Ok(data) => {
                tracing::info!("Avg min and max data has been queried from the db.");
                HttpResponse::Ok().json(data)
            }
            Err(e) => {
                tracing::error!("Failed to execute query: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_min_and_max_data(
    lake: &str,
    attr: &str,
    db_pool: &PgPool,
) -> Result<Data, sqlx::Error> {
    let mut query: sqlx::QueryBuilder<Postgres> =
        sqlx::QueryBuilder::new("SELECT fish_type.name, fish_type.anishinaabe_name, ");
    query.push(attr);
    query.push(" as value FROM fish JOIN fish_type ON fish.fish_type_id=fish_type.id WHERE (");
    query.push(attr);
    query.push("=(SELECT MAX(");
    query.push(attr);
    query.push(") FROM fish WHERE lake=");
    query.push_bind(lake);
    query.push(") AND fish.lake=");
    query.push_bind(lake);
    query.push(") OR (");
    query.push(attr);
    query.push("=(SELECT MIN(");
    query.push(attr);
    query.push(") FROM fish where lake=");
    query.push_bind(lake);
    query.push(") AND fish.lake=");
    query.push_bind(lake);
    query.push(") ORDER BY ");
    query.push(attr);
    query.push(";");
    let stream = query.build_query_as::<Fish>();
    let data = stream.fetch_all(db_pool).await.map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    let data = Data {
        min: data.first().cloned(),
        max: data.last().cloned(),
    };

    Ok(data)
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_min_and_max_of_avg_data(attr: &str, db_pool: &PgPool) -> Result<Data, sqlx::Error> {
    let mut query: sqlx::QueryBuilder<Postgres> =
        sqlx::QueryBuilder::new("SELECT fish_type.name, fish_type.anishinaabe_name, CAST(AVG(");
    query.push(attr);
    query.push(
        ") AS FLOAT4) AS value FROM fish JOIN fish_type ON fish.fish_type_id=fish_type.id
        GROUP BY fish_type.name, fish_type.anishinaabe_name ORDER BY value;",
    );
    let stream = query.build_query_as::<Fish>();
    let data = stream.fetch_all(db_pool).await.map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    let data = Data {
        min: data.first().cloned(),
        max: data.last().cloned(),
    };

    Ok(data)
}
