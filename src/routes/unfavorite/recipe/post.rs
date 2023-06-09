use crate::utils::get_user_id;
use actix_web::{post, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct RecipeUuid {
    uuid: Uuid,
}

#[tracing::instrument(name = "Unfavoriting a recipe", skip(uuid, db_pool))]
#[post("/recipe/{uuid}")]
pub async fn unfavorite_recipe(
    uuid: web::Path<RecipeUuid>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_id(req)?;
    match unfavorite_recipe_db(&db_pool, user_id, uuid.uuid).await {
        Ok(_) => {
            tracing::info!("Recipe has been unfavorited.");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(name = "Unfavoriting the recipe in the database", skip(db_pool))]
async fn unfavorite_recipe_db(
    db_pool: &PgPool,
    user_id: Uuid,
    recipe_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM user_recipe 
        WHERE 
            user_id = $1
        AND
            recipe_id = $2;
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
