use crate::routes::admin::recipe::RecipeData;
use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Saving new recipe data", skip(data, db_pool))]
#[post("/recipe/")]
pub async fn new_recipe(data: web::Json<RecipeData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let recipe_id = Uuid::new_v4();
    match save_new_recipe(&db_pool, data, recipe_id).await {
        Ok(_) => {
            tracing::info!("New recipe has been saved to the database.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving recipe data to the database", skip(db_pool, data))]
async fn save_new_recipe(
    db_pool: &PgPool,
    data: web::Json<RecipeData>,
    recipe_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO recipe (id, name, ingredients, steps)
        VALUES ($1, $2, $3, $4)
        "#,
        recipe_id,
        data.name,
        &data.ingredients,
        &data.steps
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
