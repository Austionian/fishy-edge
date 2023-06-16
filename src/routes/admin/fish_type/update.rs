use crate::routes::structs::FishType;
use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FishTypeId {
    uuid: Uuid,
}

#[tracing::instrument(name = "Updating a fish type.", skip(data, db_pool))]
#[post("/{uuid}")]
pub async fn update_fish_type(
    fish_type_id: web::Path<FishTypeId>,
    data: web::Json<FishType>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match update_fish_type_db(&db_pool, fish_type_id.uuid, data).await {
        Ok(_) => {
            tracing::info!("Fish type has been updated.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving fish type data to the database",
    skip(db_pool, fish_type_id, data)
)]
async fn update_fish_type_db(
    db_pool: &PgPool,
    fish_type_id: Uuid,
    data: web::Json<FishType>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE fish_type
        SET 
            name = $1,
            anishinaabe_name = $2,
            about = $3
        WHERE id = $4;
        "#,
        data.name,
        data.anishinaabe_name,
        data.about,
        fish_type_id
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
