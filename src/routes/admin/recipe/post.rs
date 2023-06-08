use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct RecipeUuid {
    uuid: Uuid,
}

#[derive(serde::Deserialize)]
pub struct RecipeData {
    name: String,
    ingredients: Vec<String>,
    steps: Vec<String>,
}

#[tracing::instrument(name = "Updating recipe data", skip(uuid, data, db_pool))]
#[post("/recipe/{uuid}")]
pub async fn update_recipe(
    uuid: web::Path<RecipeUuid>,
    data: web::Json<RecipeData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match update_recipe_db(&db_pool, uuid.uuid, data).await {
        Ok(data) => {
            tracing::info!("Fish type data has been queried from the db.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving recipe data to the database", skip(db_pool, data))]
async fn update_recipe_db(
    db_pool: &PgPool,
    recipe_uuid: Uuid,
    data: web::Json<RecipeData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE recipe
        SET 
           name = $1,
           ingredients = $2,
           steps = $3
        WHERE id = $4
        "#,
        data.name,
        &data.ingredients,
        &data.steps,
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
