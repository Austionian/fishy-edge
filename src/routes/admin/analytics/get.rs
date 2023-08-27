use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;

#[derive(serde::Serialize)]
pub struct Data {
    pub(crate) user_data: Vec<UserData>,
    pub(crate) number_of_registered_users: usize,
}

#[derive(serde::Serialize)]
pub struct UserData {
    email: String,
    created_at: Option<chrono::DateTime<Utc>>,
    latest_login: Option<chrono::DateTime<Utc>>,
}

#[tracing::instrument(name = "Fetching analytic data.", skip(db_pool))]
#[get("/")]
pub async fn get_analytics(db_pool: web::Data<PgPool>) -> HttpResponse {
    match analytics(&db_pool).await {
        Ok(data) => {
            tracing::info!("Analytic data has been fetched.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Querying the database for analytic data.", skip(db_pool))]
async fn analytics(db_pool: &PgPool) -> Result<Data, sqlx::Error> {
    let number_of_registered_users = sqlx::query!(
        r#"
        SELECT count(email) FROM users;
        "#
    )
    .fetch_one(db_pool)
    .await
    .map(|row| match row.count {
        Some(count) => count as usize,
        None => 0,
    })?;

    let user_data = sqlx::query_as!(
        UserData,
        r#"
        SELECT 
            email,
            created_at,
            latest_login
        FROM users;
        "#
    )
    .fetch_all(db_pool)
    .await?;

    Ok(Data {
        user_data,
        number_of_registered_users,
    })
}
