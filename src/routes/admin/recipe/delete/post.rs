use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct RecipeUuid {
    uuid: Uuid,
}

#[tracing::instrument(name = "Deleting recipe data", skip(uuid, db_pool))]
#[post("/delete/{uuid}")]
pub async fn delete_recipe(
    uuid: web::Path<RecipeUuid>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match delete_recipe_db(&db_pool, uuid.uuid).await {
        Ok(_) => {
            tracing::info!("Recipe has been deleted.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Deleting recipe data from the database", skip(db_pool))]
async fn delete_recipe_db(db_pool: &PgPool, recipe_uuid: Uuid) -> Result<(), sqlx::Error> {
    tracing::info!("Deleting recipe from fish type join table.");
    sqlx::query!(
        r#"
        DELETE FROM fishtype_recipe
        WHERE recipe_id = $1;
        "#,
        recipe_uuid
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    tracing::info!("Deleting recipe.");
    sqlx::query!(
        r#"
        DELETE FROM recipe
        WHERE id = $1;
        "#,
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
