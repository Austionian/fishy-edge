use actix_web::{post, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct RecipeUuid {
    uuid: Uuid,
}

#[tracing::instrument(name = "Favoriting a recipe", skip(uuid, db_pool))]
#[post("/recipe/{uuid}")]
pub async fn favorite_recipe(
    uuid: web::Path<RecipeUuid>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = Uuid::parse_str(
        req.cookie("user_id")
            .ok_or(actix_web::error::ErrorBadRequest("No user id provided."))?
            .value(),
    )
    .map_err(actix_web::error::ErrorBadRequest)?;
    match favorite_recipe_db(&db_pool, user_id, uuid.uuid).await {
        Ok(_) => {
            tracing::info!("Recipe has been favorited.");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(name = "Favoriting the recipe in the database", skip(db_pool))]
async fn favorite_recipe_db(
    db_pool: &PgPool,
    user_id: Uuid,
    recipe_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_recipe (user_id, recipe_id)
        VALUES ($1, $2);
        "#,
        user_id,
        recipe_uuid
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
