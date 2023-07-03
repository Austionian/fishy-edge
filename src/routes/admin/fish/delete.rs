use actix_web::{delete, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

#[tracing::instrument(name = "Deleting fish data", skip(uuid, db_pool))]
#[delete("/{uuid}")]
pub async fn delete_fish(uuid: web::Path<FishUuid>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match delete_fish_db(&db_pool, uuid.uuid).await {
        Ok(_) => {
            tracing::info!("Fish has been deleted.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Deleting fish data from the database", skip(db_pool))]
async fn delete_fish_db(db_pool: &PgPool, fish_uuid: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM fish
        WHERE id = $1;
        "#,
        fish_uuid
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
