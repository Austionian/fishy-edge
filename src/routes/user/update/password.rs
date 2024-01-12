use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::utils::e500;
use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: Uuid,
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

/// An endpoint to update a user's password.
/// It expects the `user_id`, `current_password`, `new_password`, and
/// `new_password_check` to be included as form data.
pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = form.user_id;
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let email = get_email(user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        email,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => Ok(HttpResponse::BadRequest().finish()),
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }
    crate::authentication::change_password(user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())
}

async fn get_email(user_id: Uuid, pool: &PgPool) -> Result<String, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT email
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(row.email)
}
