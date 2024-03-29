use crate::{
    routes::{Fish, Recipe},
    utils::get_optional_user_id,
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FishResponse {
    pub fish_data: Fish,
    pub recipe_data: Vec<Recipe>,
    pub is_favorite: bool,
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
///       "fish_data": {
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
///       "recipe_data": {
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
///       },
///       is_favorite: bool
/// }
///```
///
#[tracing::instrument(name = "Retreving fish data", skip(uuid, db_pool))]
#[get("/fish/{uuid}")]
pub async fn fish(
    uuid: web::Path<FishUuid>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_optional_user_id(req)?;
    match get_all_fish_data(&db_pool, uuid.uuid, user_id).await {
        Ok(data) => {
            tracing::info!("Fish type data has been queried from the db.");
            Ok(HttpResponse::Ok().json(data))
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => Ok(HttpResponse::BadRequest().finish()),
                _ => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
    }
}

async fn get_all_fish_data(
    db_pool: &PgPool,
    fish_uuid: Uuid,
    user_id: Option<Uuid>,
) -> Result<FishResponse, sqlx::Error> {
    let fish_data = get_fish_data(db_pool, fish_uuid).await?;
    let recipe_data = get_recipe_data(db_pool, fish_data.fish_type_id).await?;
    let is_favorite = match user_id {
        Some(user_id) => get_is_favorite(db_pool, fish_data.fish_type_id, user_id).await?,
        None => false,
    };

    Ok(FishResponse {
        fish_data,
        recipe_data,
        is_favorite,
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
            fish.lake,
            fish_type.about
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
            steps,
            image_url
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

/// Finds whether the selected fish has been favorited by the user.
#[tracing::instrument(
    name = "Querying for is favorited fish",
    skip(fish_type_id, user_id, db_pool)
)]
pub async fn get_is_favorite(
    db_pool: &PgPool,
    fish_type_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let is_favorite = sqlx::query!(
        r#"
        SELECT *
        FROM user_fishtype
        WHERE
            user_id = $1
        AND
            fishtype_id = $2;
        "#,
        user_id,
        fish_type_id
    )
    .fetch_optional(db_pool)
    .await?
    .is_some();

    Ok(is_favorite)
}
