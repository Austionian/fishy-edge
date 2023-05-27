use crate::utils::e500;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: Uuid,
    image_url: String,
}

/// Saves the users image url to the user object.
/// Requires the user_id and image_url to be in the body's request.
#[tracing::instrument(
    name="Saving user's image",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.user_id
        )
    )]
pub async fn post_image(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match update_user_image(&db_pool, form).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e500(e)),
    }
}

#[tracing::instrument(name = "Saving user details to the db.", skip(db_pool, form))]
async fn update_user_image(db_pool: &PgPool, form: web::Form<FormData>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET image_url=$1 
        WHERE users.id=$2
        "#,
        form.image_url,
        form.user_id,
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
