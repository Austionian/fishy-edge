use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct AllFishData {
    fish_id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
    woodland_fish_image: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RecipeData {
    recipe_id: Uuid,
    recipe_name: String,
}

#[derive(serde::Serialize)]
pub enum QueryResult {
    FishResult(Vec<AllFishData>),
    RecipeResult(Vec<RecipeData>),
}

#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
pub async fn query(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_fish_data(&db_pool).await {
        Ok(mut data) => {
            tracing::info!("All fish type data has been queried from the db.");
            match get_recipe_data(&db_pool).await {
                Ok(mut recipe_data) => {
                    data.append(&mut recipe_data);
                    tracing::info!("All recipe data has been queried from the db.");
                    HttpResponse::Ok().json(data)
                }
                Err(e) => {
                    tracing::info!("Failed to execute query on recipes: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to execute query on fish: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Querying the database for fish", skip(db_pool))]
pub async fn get_fish_data(db_pool: &PgPool) -> Result<Vec<QueryResult>, sqlx::Error> {
    let data = sqlx::query_as!(
        AllFishData,
        r#"
        SELECT 
            fish.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image
        FROM fish
        JOIN fish_type
        ON fish.fish_type_id=fish_type.id
        WHERE fish.lake='Store';
        "#,
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    let data = vec![QueryResult::FishResult(data)];
    Ok(data)
}

#[tracing::instrument(name = "Querying the database for recipes", skip(db_pool))]
pub async fn get_recipe_data(db_pool: &PgPool) -> Result<Vec<QueryResult>, sqlx::Error> {
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

    let data = vec![QueryResult::RecipeResult(data)];
    Ok(data)
}
