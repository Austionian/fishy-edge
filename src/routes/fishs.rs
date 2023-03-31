use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Serialize)]
pub struct FishType {
    name: String,
    s3_fish_image: Option<String>,
    s3_woodland_image: Option<String>,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
}

#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
pub async fn fishs(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_fish_data(&db_pool).await {
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
pub async fn get_fish_data(db_pool: &PgPool) -> Result<Vec<FishType>, sqlx::Error> {
    let data = sqlx::query_as!(
        FishType,
        r#"
        SELECT name, s3_fish_image, s3_woodland_image, anishinaabe_name, fish_image FROM fish_type
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
