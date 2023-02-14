use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.name
        )
    )]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&db_pool, &form).await {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new subscriber details to the db.", skip(form, db_pool))]
pub async fn insert_subscriber(db_pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO fish_type (id, name)
        VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        form.name
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;
    Ok(())
}
