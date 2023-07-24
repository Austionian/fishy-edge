use crate::routes::Recipe;
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
struct Fish {
    pub id: Uuid,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub lake: String,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub mercury: Option<f32>,
    pub omega_3: Option<f32>,
    pub omega_3_ratio: Option<f32>,
    pub pcb: Option<f32>,
    pub protein: Option<f32>,
    pub recipes: Option<Vec<Uuid>>,
    pub date_sampled: Option<chrono::NaiveDateTime>,
}

#[derive(serde::Serialize)]
pub struct Everything {
    fishs: Vec<Fish>,
    recipes: Vec<Recipe>,
}

/// Returns a JSON with all fish and all recipes.
///
/// # Example
///
/// `.../everything`
///
///```json
/// {
///     "fishs": [
///      {
///       "id": "1fe5c906-d09d-11ed-afa1-0242ac120022",
///       "name": "Herring",
///       ...
///       "recipes": [
///          "1fe5c906-d09d-11ed-afa1-0242ac120022", "1fe5c906-d09d-11ed-afa1-0242ac120022"
///       ]
///      },
///       ...
///     ],
///     "recipes": [
///      {
///       "id": "1fe5c906-d09d-11ed-afa1-0242ac120022",
///       "name": "Butter Battered Cod",
///       ...
///      },
///     ],
///     ...
/// }
///```
///
#[tracing::instrument(name = "Retreving all fish data", skip(db_pool))]
#[get("/everything")]
pub async fn everything(db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_everything(&db_pool).await {
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

async fn get_everything(db_pool: &PgPool) -> Result<Everything, sqlx::Error> {
    let fishs = get_fish_data(db_pool).await?;
    let recipes = get_recipe_data(db_pool).await?;

    Ok(Everything { fishs, recipes })
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_fish_data(db_pool: &PgPool) -> Result<Vec<Fish>, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT
            fish.id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish.lake,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            fish.pcb,
            fish.protein,
            fish.mercury,
            fish.omega_3_ratio,
            fish.omega_3,
            fish.date_sampled,
            array(
                SELECT recipe_id
                FROM fishtype_recipe
                WHERE fishtype_recipe.fishtype_id=fish_type.id
                ) as recipes
        FROM fish 
        JOIN fish_type ON fish.fish_type_id=fish_type.id
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
async fn get_recipe_data(db_pool: &PgPool) -> Result<Vec<Recipe>, sqlx::Error> {
    let data = sqlx::query_as!(
        Recipe,
        r#"
        SELECT 
            id,
            name,
            ingredients,
            steps
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
