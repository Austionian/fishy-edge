use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct FishType {
    id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
    about: String,
}

#[tracing::instrument(name = "Retreving all fish types.", skip(db_pool))]
#[get("/fish_types")]
pub async fn fish_types(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_fish_type_data(&db_pool).await {
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
async fn get_fish_type_data(db_pool: &PgPool) -> Result<Vec<FishType>, sqlx::Error> {
    let data = sqlx::query_as!(
        FishType,
        r#"
        SELECT
            id,
            name,
            anishinaabe_name,
            about
        FROM fish_type;
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
