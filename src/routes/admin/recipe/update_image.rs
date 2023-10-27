use actix_web::{put, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct RecipeUuid {
    uuid: Uuid,
}

#[derive(serde::Deserialize)]
pub struct RecipeImageData {
    image_url: String,
}

#[tracing::instrument(name = "Updating a recipe image.", skip(data, db_pool))]
#[put("/{uuid}/image")]
pub async fn update_recipe_image(
    recipe_id: web::Path<RecipeUuid>,
    data: web::Json<RecipeImageData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match update_recipe_image_db(&db_pool, recipe_id.uuid, data).await {
        Ok(_) => {
            tracing::info!("Recipe image has been updated.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new recipe image url to db.", skip(db_pool, data))]
async fn update_recipe_image_db(
    db_pool: &PgPool,
    recipe_id: Uuid,
    data: web::Json<RecipeImageData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE recipe
        SET image_url = $1
        WHERE id = $2;
        "#,
        data.image_url,
        recipe_id,
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
