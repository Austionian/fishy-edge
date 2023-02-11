use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = uuid::Uuid::new_v4();
    let request_span = tracing::info_span!(
    "Adding a new subscriber",
    %request_id,
    subscriber_name = %form.name,
    );
    tracing::info!(
        "request_id: {} - Adding '{}' as a new subscriber!",
        request_id,
        form.name
    );
    match sqlx::query!(
        r#"
        INSERT INTO fish_type (id, name)
        VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        form.name
    )
    .execute(db_pool.get_ref())
    .await
    {
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
