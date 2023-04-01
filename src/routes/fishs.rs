use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct AllFishData {
    id: Uuid,
    fish_id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
    woodland_fish_image: Option<String>,
    s3_fish_image: Option<String>,
    s3_woodland_image: Option<String>,
    recipe_name: String,
    ingredients: Option<Vec<String>>,
    steps: Option<Vec<String>>,
}

#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
pub async fn fishs(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_fish_data(&db_pool).await {
        Ok(data) => {
            tracing::info!("All fish type data has been queried from the db.");
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
pub async fn get_fish_data(db_pool: &PgPool) -> Result<Vec<AllFishData>, sqlx::Error> {
    let data = sqlx::query_as!(
        AllFishData,
        r#"
        SELECT 
            fish_type.id,
            fish.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            recipe.name as recipe_name,
            recipe.ingredients,
            recipe.steps
        FROM fish_type
        JOIN fishtype_recipe
        ON fish_type.id=fishtype_recipe.fishtype_id
        JOIN recipe
        ON fishtype_recipe.recipe_id=recipe.id
        JOIN fish
        ON fish_type.id=fish.fish_type_id
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(data)
}
