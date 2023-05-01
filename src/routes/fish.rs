use crate::routes::{Fish, Recipe};
use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

#[derive(serde::Serialize)]
pub struct FishData {
    fish_data: Fish,
    recipe_data: Vec<Recipe>,
}

/// Retrives data for a fish specified by its uuid. If an invalid uuid is given
/// a 400 Bad Request will be returned.
///
/// # Example
///
/// `/fish/1fe5c906-d09d-11ed-afa1-0242ac120002`
///
///```json
/// {
///       fish_data: {
///         "name": "Herring",
///         "anishinaabe_name": "Okewis",
///         "fish_image": "herring.png",
///         "woodland_fish_image": "woodlandherring.webp",
///         "s3_fish_image": "",
///         "s3_woodland_image": "",
///         "mercury": 0.032,
///         "omega_3": 0.212,
///         "omega_3_ratio": 8.12,
///         "pcb": 0.0002,
///         "protein": 21.1
///       },
///       recipe_data: {
///         [
///           "id": uuid,
///           "name": "Fish Stew",
///           "ingredients": [
///             "1 Fish",
///             ...
///           ],
///           "steps": [
///             "Add fish to stew",
///             ...
///           ]
///         ],
///         ...
///       }
///
/// }
///```
///
#[tracing::instrument(name = "Retreving fish data", skip(uuid, db_pool))]
#[get("/fish/{uuid}")]
pub async fn fish(uuid: web::Path<FishUuid>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_all_fish_data(&db_pool, uuid.uuid).await {
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

async fn get_all_fish_data(db_pool: &PgPool, fish_uuid: Uuid) -> Result<FishData, sqlx::Error> {
    let fish_data = get_fish_data(db_pool, fish_uuid).await?;
    let recipe_data = get_recipe_data(db_pool, fish_data.fish_type_id).await?;

    Ok(FishData {
        fish_data,
        recipe_data,
    })
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_fish_data(db_pool: &PgPool, fish_uuid: Uuid) -> Result<Fish, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT 
            fish.id as fish_id,
            fish.fish_type_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            fish.mercury,
            fish.omega_3,
            fish.omega_3_ratio,
            fish.pcb,
            fish.protein,
            fish.lake
        FROM fish_type
        INNER JOIN fish
        ON fish_type.id=fish.fish_type_id
        WHERE fish.id = $1;
        "#,
        fish_uuid
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}

#[tracing::instrument(
    name = "Querying the database for recipes",
    skip(fish_type_id, db_pool)
)]
async fn get_recipe_data(db_pool: &PgPool, fish_type_id: Uuid) -> Result<Vec<Recipe>, sqlx::Error> {
    let data = sqlx::query_as!(
        Recipe,
        r#"
        SELECT
            id,
            name,
            ingredients,
            steps
        FROM recipe
        WHERE recipe.id
        IN (
            SELECT 
                recipe_id
            FROM fishtype_recipe
            WHERE fishtype_id = $1
        );
        "#,
        fish_type_id
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
