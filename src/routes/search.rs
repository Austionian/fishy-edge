use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct FishData {
    fish_id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
    woodland_fish_image: Option<String>,
    s3_fish_image: Option<String>,
    s3_woodland_image: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RecipeData {
    recipe_id: Uuid,
    recipe_name: String,
}

#[derive(serde::Serialize)]
pub struct SearchResult {
    fish_result: Vec<FishData>,
    recipe_result: Vec<RecipeData>,
}

/// Returns a JSON of all store fish and recipes.
///
/// # Example
///
/// `.../search`
///
///```json
/// {
///   fish_result: [
///     {
///       "fish_id": "1fe5c906-d09d-11ed-afa1-0242ac120022",
///       "name": "Herring",
///       // ...
///     },
///     // ...
///   ],
///   recipe_result: [
///     {
///       "id": "1fe5c906-d09d-11ed-afa1-0242ac120002",
///       "name": "Simple Salmon Recipe"
///     },
///     // ...
///   ]
///     ...
/// }
///```
#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
pub async fn search(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_search_results(&db_pool).await {
        Ok(result) => {
            tracing::info!("All data has been queried from the db");
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            tracing::info!("Failed to execute query on recipes: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_search_results(db_pool: &PgPool) -> Result<SearchResult, sqlx::Error> {
    let fish_result = get_fish_data(db_pool).await?;
    let recipe_result = get_recipe_data(db_pool).await?;

    Ok(SearchResult {
        fish_result,
        recipe_result,
    })
}

#[tracing::instrument(name = "Querying the database for fish", skip(db_pool))]
async fn get_fish_data(db_pool: &PgPool) -> Result<Vec<FishData>, sqlx::Error> {
    let data = sqlx::query_as!(
        FishData,
        r#"
        SELECT 
            fish_type.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image
        FROM fish_type;
        "#,
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}

#[tracing::instrument(name = "Querying the database for recipes", skip(db_pool))]
async fn get_recipe_data(db_pool: &PgPool) -> Result<Vec<RecipeData>, sqlx::Error> {
    let data = sqlx::query_as!(
        RecipeData,
        r#"
        SELECT 
            id as recipe_id,
            name as recipe_name
        FROM recipe
        "#,
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
