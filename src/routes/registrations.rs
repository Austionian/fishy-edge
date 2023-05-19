use crate::authentication::compute_password_hash;
use crate::utils::e500;
use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
}

/// Adds a new user to the database and returns a 200 OK response on success.
/// Expects the user's email and password to be included in the form data.
#[tracing::instrument(
    name="Registering a new user",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.email
        )
    )]
pub async fn register(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = &form.email;
    let password = &form.password;
    let password_hash = compute_password_hash(password.clone()).map_err(e500)?;
    match insert_user(&db_pool, email.to_string(), password_hash).await {
        Ok(_) => {
            tracing::info!("New user details have been saved.");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(
    name = "Saving new user details to the db.",
    skip(email, password_hash, db_pool)
)]
async fn insert_user(
    db_pool: &PgPool,
    email: String,
    password_hash: Secret<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        email,
        password_hash.expose_secret()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
