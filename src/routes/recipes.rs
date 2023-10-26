use crate::routes::Recipe;
use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;

/// Retrives data for all recipes.
///
/// # Example
///
/// `/recipe/
///
///```json
/// {
///   [
///     "id": uuid,
///     "name": "Fish Stew",
///     "ingredients": [
///       "1 Fish",
///       ...
///     ],
///     "steps": [
///       "Add fish to stew",
///       ...
///     ]
///   ],
///   ...
/// }
///```
///
#[tracing::instrument(name = "Retreving all recipe data", skip(db_pool))]
#[get("/recipe/")]
pub async fn recipes(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_recipe_data(&db_pool).await {
        Ok(data) => {
            tracing::info!("Recipe data has been queried from the db.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Querying the database for recipes", skip(db_pool))]
async fn get_recipe_data(db_pool: &PgPool) -> Result<Vec<Recipe>, sqlx::Error> {
    let data = sqlx::query_as!(
        Recipe,
        r#"
        SELECT 
            id,
            name,
            ingredients,
            steps,
            image_url
        FROM recipe
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
