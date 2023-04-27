use crate::routes::structs::Recipe;
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Fish {
    pub fish_id: Uuid,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub mercury: Option<f64>,
    pub omega_3: Option<f64>,
    pub omega_3_ratio: Option<f64>,
    pub pcb: Option<f64>,
    pub protein: Option<f64>,
}

#[derive(serde::Serialize)]
pub struct FishData {
    fish_data: Fish,
    recipe_data: Vec<Recipe>,
}

#[derive(serde::Deserialize)]
pub struct FishQuery {
    fishtype_id: Uuid,
}

/// Returns a JSON object with a single fish type and its averages.
///
/// # Example
///
/// `.../fish_avg?fishtype_id=1fe5c906-d09d-11ed-afa1-0242ac120022`
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
#[tracing::instrument(name = "Retreving all fish data", skip(query, db_pool))]
#[get("/fish_avg")]
pub async fn fish_avg(query: web::Query<FishQuery>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_all_fish_data(query.fishtype_id, &db_pool).await {
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

async fn get_all_fish_data(fish_uuid: Uuid, db_pool: &PgPool) -> Result<FishData, sqlx::Error> {
    let fish_data = get_fish_data(fish_uuid, db_pool).await?;
    let recipe_data = get_recipe_data(fish_uuid, db_pool).await?;

    Ok(FishData {
        fish_data,
        recipe_data,
    })
}

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
async fn get_fish_data(fishtype_id: Uuid, db_pool: &PgPool) -> Result<Fish, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT
            fish_type.id as fish_id,
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            AVG(pcb) as pcb,
            AVG(protein) as protein,
            AVG(mercury) as mercury,
            AVG(omega_3_ratio) as omega_3_ratio,
            AVG(omega_3) as omega_3
        FROM fish 
        JOIN fish_type ON fish.fish_type_id=fish_type.id
        WHERE fish_type.id=$1
        GROUP BY fish_type.id;
        "#,
        fishtype_id
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
async fn get_recipe_data(fish_type_id: Uuid, db_pool: &PgPool) -> Result<Vec<Recipe>, sqlx::Error> {
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
