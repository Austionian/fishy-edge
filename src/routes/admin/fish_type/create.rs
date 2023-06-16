use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct NewFishType {
    name: String,
    anishinaabe_name: String,
    fish_image: String,
    woodland_fish_image: Option<String>,
    about: String,
}

#[tracing::instrument(name = "Creating a new fish type.", skip(data, db_pool))]
#[post("/")]
pub async fn new_fish_type(
    data: web::Json<NewFishType>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let fish_id = Uuid::new_v4();
    match new_fish_type_db(&db_pool, fish_id, data).await {
        Ok(_) => {
            tracing::info!("New fish has been added.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new fish type data to the database",
    skip(db_pool, data)
)]
async fn new_fish_type_db(
    db_pool: &PgPool,
    fish_id: Uuid,
    data: web::Json<NewFishType>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO fish_type (
            id,
            name,
            anishinaabe_name,
            s3_fish_image,
            s3_woodland_image,
            about
        )
        VALUES (
            $1, $2, $3, $4, $5, $6
        );
        "#,
        fish_id,
        data.name,
        data.anishinaabe_name,
        data.fish_image,
        data.woodland_fish_image,
        data.about
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
