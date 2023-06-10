use crate::utils::get_user_id;
use actix_web::{post, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

#[tracing::instrument(name = "Favoriting a fish", skip(uuid, db_pool))]
#[post("/fish/{uuid}")]
pub async fn favorite_fish(
    uuid: web::Path<FishUuid>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_id(req)?;
    match favorite_fish_db(&db_pool, user_id, uuid.uuid).await {
        Ok(_) => {
            tracing::info!("Fish has been favorited.");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(name = "Favoriting the fish in the database", skip(db_pool))]
async fn favorite_fish_db(
    db_pool: &PgPool,
    user_id: Uuid,
    recipe_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_fishtype (user_id, fishtype_id)
        VALUES ($1, $2);
        "#,
        user_id,
        recipe_uuid
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
