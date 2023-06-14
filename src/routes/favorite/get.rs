use crate::utils::get_user_id;
use actix_web::{get, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct FavoriteFish {
    id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct FavoriteRecipe {
    id: Uuid,
    name: String,
}

#[derive(serde::Serialize)]
pub struct Favorites {
    fishs: Vec<FavoriteFish>,
    recipes: Vec<FavoriteRecipe>,
}

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
        FavoriteFish,
        r#"
        SELECT
            id,
            name,
            anishinaabe_name
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
        FavoriteRecipe,
        r#"
        SELECT
            id,
            name
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
