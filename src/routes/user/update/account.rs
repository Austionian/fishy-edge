use crate::utils::e500;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
}

/// An endpoint to update a user's account information.
/// It requires the `user_id` and `email`,
/// `first_name`, and `last_name` to be included as form data.
pub async fn update_account(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match update_account_db(form, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(e500(e).into()),
    }
}

#[tracing::instrument(name = "Saving user account details to the db.", skip(pool, form))]
async fn update_account_db(form: web::Form<FormData>, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET email=$1, first_name=$2, last_name=$3 
        WHERE id = $4
        "#,
        form.email,
        form.first_name,
        form.last_name,
        form.user_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
