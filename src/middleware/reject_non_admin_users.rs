use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error;
use actix_web::web;
use actix_web_lab::middleware::Next;
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::e500;

pub async fn reject_non_admin_users(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let user_id = Uuid::parse_str(
        req.cookie("user_id")
            .ok_or(e500("No user_id cookie included with the request."))?
            .value(),
    )
    .map_err(error::ErrorBadRequest)?;

    let db_pool = req
        .app_data::<web::Data<PgPool>>()
        .ok_or(e500("Unable to find attached pool."))?;

    match get_user_is_admin(db_pool, user_id).await {
        Ok(true) => next.call(req).await,
        Ok(false) | Err(_) => {
            let e = anyhow::anyhow!("The user does not have admin rights.");
            Err(error::ErrorUnauthorized(e))
        }
    }
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_user_is_admin(db_pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let data = sqlx::query!(
        r#"
        SELECT 
            is_admin
        FROM users
        WHERE id = $1;
        "#,
        user_id
    )
    .fetch_one(db_pool)
    .await
    .map(|row| row.is_admin.unwrap_or(false))
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
