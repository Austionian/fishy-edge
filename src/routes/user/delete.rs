use crate::utils::e500;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserUuid {
    uuid: Uuid,
}

/// Deletes a user's profile from the db.
#[tracing::instrument(name = "Deleting user's info", skip(uuid, db_pool))]
pub async fn delete_user(
    uuid: web::Path<UserUuid>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match delete_user_from_db(&db_pool, uuid).await {
        Ok(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        Err(e) => Err(e500(e)),
    }
}

#[tracing::instrument(name = "Deleting user details from the db.", skip(db_pool, uuid))]
async fn delete_user_from_db(
    db_pool: &PgPool,
    uuid: web::Path<UserUuid>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id=$1
        "#,
        uuid.uuid
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
