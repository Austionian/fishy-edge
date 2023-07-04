use crate::{routes::Recipe, utils::get_optional_user_id};
use actix_web::{get, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct RecipeUuid {
    uuid: Uuid,
}

#[derive(serde::Serialize)]
pub struct RecipeResponse {
    data: Recipe,
    is_favorite: bool,
}

/// Retrives data for a recipe specified by its uuid. If an invalid uuid is given
/// a 400 Bad Request will be returned.
///
/// # Example
///
/// `/recipe/1fe5c906-d09d-11ed-afa1-0242ac120002`
///
///```json
/// {
///    "id": uuid,
///    "name": "Fish Stew",
///    "ingredients": [
///      "1 Fish",
///      ...
///    ],
///    "steps": [
///      "Add fish to stew",
///      ...
///    ]
/// }
///```
///
#[tracing::instrument(name = "Retreving recipe data", skip(uuid, db_pool))]
#[get("/recipe/{uuid}")]
pub async fn recipe(
    uuid: web::Path<RecipeUuid>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_optional_user_id(req)?;
    match get_recipe_data(&db_pool, uuid.uuid, user_id).await {
        Ok(data) => {
            tracing::info!("Recipe data has been queried from the db.");
            Ok(HttpResponse::Ok().json(data))
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[tracing::instrument(name = "Querying the database for a recipe", skip(db_pool))]
async fn get_recipe_data(
    db_pool: &PgPool,
    recipe_uuid: Uuid,
    user_id: Option<Uuid>,
) -> Result<RecipeResponse, sqlx::Error> {
    let data = sqlx::query_as!(
        Recipe,
        r#"
        SELECT 
            id,
            name,
            ingredients,
            steps
        FROM recipe
        WHERE id = $1;
        "#,
        recipe_uuid
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    let is_favorite = match user_id {
        Some(user_id) => sqlx::query!(
            r#"
            SELECT *
            FROM user_recipe
            WHERE user_id = $1
            AND recipe_id = $2; 
            "#,
            user_id,
            data.id
        )
        .fetch_optional(db_pool)
        .await?
        .is_some(),
        None => false,
    };

    Ok(RecipeResponse { data, is_favorite })
}
