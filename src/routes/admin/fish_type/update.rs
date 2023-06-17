use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FishTypeId {
    uuid: Uuid,
}

#[derive(serde::Deserialize)]
pub struct UpdateFishType {
    name: String,
    anishinaabe_name: String,
    recipe: Option<Vec<Uuid>>,
    about: String,
}

#[tracing::instrument(name = "Updating a fish type.", skip(data, db_pool))]
#[post("/{uuid}")]
pub async fn update_fish_type(
    fish_type_id: web::Path<FishTypeId>,
    data: web::Json<UpdateFishType>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match update_fish_type_db(&db_pool, fish_type_id.uuid, data).await {
        Ok(_) => {
            tracing::info!("Fish type has been updated.");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(
    name = "Saving fish type data to the database",
    skip(db_pool, fish_type_id, data)
)]
async fn update_fish_type_db(
    db_pool: &PgPool,
    fish_type_id: Uuid,
    data: web::Json<UpdateFishType>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE fish_type
        SET 
            name = $1,
            anishinaabe_name = $2,
            about = $3
        WHERE id = $4;
        "#,
        data.name,
        data.anishinaabe_name,
        data.about,
        fish_type_id
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    if let Some(recipes) = &data.recipe {
        delete_recipes_fish_type(db_pool, fish_type_id).await?;

        tracing::info!("Inserting recipes into user_recipe join table.");
        for recipe_id in recipes {
            insert_recipes_fish_type(db_pool, fish_type_id, *recipe_id).await?;
        }
    };

    Ok(())
}

#[tracing::instrument(
    name = "Clearing fish type data in the fishtype_recipe table.",
    skip(db_pool, fish_type_id)
)]
async fn delete_recipes_fish_type(db_pool: &PgPool, fish_type_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM fishtype_recipe
        WHERE fishtype_id = $1;
        "#,
        fish_type_id
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}

pub async fn insert_recipes_fish_type(
    db_pool: &PgPool,
    fish_type_id: Uuid,
    recipe_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO fishtype_recipe (
           fishtype_id, recipe_id 
        )
        VALUES (
            $1, $2
        );
        "#,
        fish_type_id,
        recipe_id
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
