use crate::utils::get_user_id;
use actix_web::{post, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

#[tracing::instrument(name = "Unfavoriting a fish", skip(uuid, db_pool))]
#[post("/fish/{uuid}")]
pub async fn unfavorite_fish(
    uuid: web::Path<FishUuid>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_id(req)?;
    match unfavorite_fish_db(&db_pool, user_id, uuid.uuid).await {
        Ok(_) => {
            tracing::info!("Fish has been unfavorited.");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(name = "Unfavoriting the fish in the database", skip(db_pool))]
async fn unfavorite_fish_db(
    db_pool: &PgPool,
    user_id: Uuid,
    fish_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM user_fishtype 
        WHERE 
            user_id = $1
        AND
            fishtype_id = $2;
        "#,
        user_id,
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
