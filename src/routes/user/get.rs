use crate::utils::e500;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserUuid {
    uuid: Uuid,
}

#[derive(serde::Serialize)]
pub struct UserData {
    weight: Option<i16>,
    age: Option<i16>,
    sex: Option<String>,
    plan_to_get_pregnant: Option<bool>,
    portion_size: Option<i16>,
}

/// Adds a new user to the database and returns a 200 OK response on success.
/// Expects the user's email and password to be included in the form data.
#[tracing::instrument(name = "Getting user's info", skip(uuid, db_pool))]
pub async fn get_user(
    uuid: web::Path<UserUuid>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match get_user_db(&db_pool, uuid).await {
        Ok(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        Err(e) => Err(e500(e)),
    }
}

#[tracing::instrument(name = "Getting user details from the db.", skip(db_pool, uuid))]
async fn get_user_db(db_pool: &PgPool, uuid: web::Path<UserUuid>) -> Result<UserData, sqlx::Error> {
    let user_data = sqlx::query_as!(
        UserData,
        r#"
        SELECT
            weight,
            age,
            sex,
            plan_to_get_pregnant,
            portion_size
        FROM users
        WHERE id=$1
        "#,
        uuid.uuid
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(user_data)
}
