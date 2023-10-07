use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;

#[derive(serde::Serialize)]
pub struct Data {
    pub(crate) user_data: Vec<UserData>,
    pub(crate) number_of_registered_users: usize,
    pub(crate) most_liked_fish: String,
    pub(crate) most_liked_fish_id: uuid::Uuid,
    pub(crate) fish_like_count: Option<i64>,
    pub(crate) most_liked_recipe: String,
    pub(crate) most_liked_recipe_id: uuid::Uuid,
    pub(crate) recipe_like_count: Option<i64>,
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

    let (most_liked_fish_id, most_liked_fish, fish_like_count) = sqlx::query!(
        r#"
        SELECT 
            ft.id,
            ft.name,
            like_count 
        FROM (
            SELECT 
                fishtype_id, 
                count(fishtype_id) as like_count
            FROM user_fishtype 
            GROUP BY fishtype_id 
            ORDER BY fishtype_id DESC 
            LIMIT 1) 
        AS subquery
        JOIN fish_type AS ft 
        ON subquery.fishtype_id = ft.id;
        "#
    )
    .fetch_one(db_pool)
    .await
    .map(|row| (row.id, row.name, row.like_count))?;

    let (most_liked_recipe_id, most_liked_recipe, recipe_like_count) = sqlx::query!(
        r#"
        SELECT 
            r.id,
            r.name,
            like_count 
        FROM (
            SELECT 
                recipe_id, 
                count(recipe_id) as like_count
            FROM user_recipe
            GROUP BY recipe_id
            ORDER BY recipe_id DESC 
            LIMIT 1) 
        AS subquery
        JOIN recipe AS r 
        ON subquery.recipe_id = r.id;
        "#
    )
    .fetch_one(db_pool)
    .await
    .map(|row| (row.id, row.name, row.like_count))?;

    Ok(Data {
        user_data,
        number_of_registered_users,
        most_liked_fish_id,
        most_liked_fish,
        fish_like_count,
        most_liked_recipe,
        most_liked_recipe_id,
        recipe_like_count,
    })
}
