use crate::routes::FishType;
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FishTypeId {
    uuid: Uuid,
}

#[derive(serde::Serialize)]
pub struct FishTypeResponse {
    fish: FishType,
    recipes: Vec<Uuid>,
}

#[tracing::instrument(name = "Retreving a fish type.", skip(db_pool))]
#[get("/{uuid}")]
pub async fn read_fish_type(
    db_pool: web::Data<PgPool>,
    fish_type_id: web::Path<FishTypeId>,
) -> HttpResponse {
    match get_fish_type_response(&db_pool, fish_type_id.uuid).await {
        Ok(data) => {
            tracing::info!("All fish type data has been queried from the db.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

async fn get_fish_type_response(
    db_pool: &PgPool,
    fish_type_id: Uuid,
) -> Result<FishTypeResponse, sqlx::Error> {
    let fish = get_fish_type_db(db_pool, fish_type_id).await?;
    let recipes = get_recipes_db(db_pool, fish_type_id).await?;

    Ok(FishTypeResponse { fish, recipes })
}

#[tracing::instrument(name = "Querying the database for fish type", skip(db_pool))]
async fn get_fish_type_db(db_pool: &PgPool, fish_type_id: Uuid) -> Result<FishType, sqlx::Error> {
    let data = sqlx::query_as!(
        FishType,
        r#"
        SELECT
            id,
            name,
            anishinaabe_name,
            fish_image,
            s3_fish_image,
            s3_woodland_image,
            woodland_fish_image,
            about
        FROM fish_type
        WHERE id = $1;
        "#,
        fish_type_id
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
    name = "Querying the database for fish type recipe data",
    skip(db_pool)
)]
async fn get_recipes_db(db_pool: &PgPool, fish_type_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
    let data = sqlx::query!(
        r#"
        SELECT
            recipe_id
        FROM fishtype_recipe
        WHERE fishtype_id = $1;
        "#,
        fish_type_id
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?
    .iter()
    .map(|row| row.recipe_id)
    .collect();

    Ok(data)
}
