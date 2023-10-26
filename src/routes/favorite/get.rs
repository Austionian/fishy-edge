use crate::routes::{FishType, Recipe};
use crate::utils::get_user_id;
use actix_web::{get, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Favorites {
    fishs: Vec<FishType>,
    recipes: Vec<Recipe>,
}

/// Gets a user's favorited fish and recipes.
///
/// # Example
///
/// `/favorite/`
///
///```json
/// {
///     fishs: [
///         {
///             "id": "",
///             "name": "",
///             "anishinaabe_name": "",
///             ...
///         },
///         ...
///     ],
///     recipes: [
///         {
///             "id": "",
///             "name": "",
///             "steps": [
///                 ...
///             ],
///             "ingredients": [
///                 ...
///             ]
///         },
///         ...
///     ]
///     
/// }
///```
///
#[tracing::instrument(name = "Getting favorite fish and recipes.", skip(db_pool))]
#[get("/")]
pub async fn favorites(
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_id(req)?;
    match favorites_db(&db_pool, user_id).await {
        Ok(data) => {
            tracing::info!("Favorites have been found.");
            Ok(HttpResponse::Ok().json(data))
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(name = "Getting favorites from the database.", skip(db_pool))]
async fn favorites_db(db_pool: &PgPool, user_id: Uuid) -> Result<Favorites, sqlx::Error> {
    let fishs = sqlx::query_as!(
        FishType,
        r#"
        SELECT
            id,
            name,
            anishinaabe_name,
            fish_image,
            woodland_fish_image,
            s3_fish_image,
            s3_woodland_image,
            about
        FROM fish_type
        JOIN user_fishtype ON fish_type.id = user_fishtype.fishtype_id
        WHERE user_fishtype.user_id = $1;
        "#,
        user_id,
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    let recipes = sqlx::query_as!(
        Recipe,
        r#"
        SELECT
            id,
            name,
            steps,
            ingredients,
            image_url
        FROM recipe
        JOIN user_recipe ON recipe.id = user_recipe.recipe_id
        WHERE user_recipe.user_id = $1;
        "#,
        user_id,
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(Favorites { fishs, recipes })
}
