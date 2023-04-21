use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
}

/// Adds a new user to the database and returns a 200 OK response on success.
/// Expects the user's email to be included in the form data.
#[tracing::instrument(
    name="Registering a new user",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.email
        )
    )]
pub async fn register(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_user(&db_pool, &form).await {
        Ok(_) => {
            tracing::info!("New user details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new user details to the db.", skip(form, db_pool))]
async fn insert_user(db_pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email)
        VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        form.email
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;
    Ok(())
}
