use actix_web::{get, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Fish {
    name: String,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
    woodland_fish_image: Option<String>,
    s3_fish_image: Option<String>,
    s3_woodland_image: Option<String>,
    mercury: Option<f32>,
    omega_3: Option<f32>,
    pcb: Option<f32>,
    protein: Option<f32>,
}

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

/// Retrives data for a fish specified by its uuid. If an invalid uuid is given
/// a 400 Bad Request will be returned.
///
/// e.g.: `/fish/1fe5c906-d09d-11ed-afa1-0242ac120002`
#[tracing::instrument(name = "Retreving fish data", skip(uuid, db_pool))]
#[get("/fish/{uuid}")]
pub async fn fish(uuid: web::Path<FishUuid>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_fish_data(&db_pool, uuid.uuid).await {
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

#[tracing::instrument(name = "Querying the database", skip(db_pool))]
pub async fn get_fish_data(db_pool: &PgPool, fish_uuid: Uuid) -> Result<Fish, sqlx::Error> {
    let data = sqlx::query_as!(
        Fish,
        r#"
        SELECT 
            fish_type.name,
            fish_type.anishinaabe_name,
            fish_type.fish_image,
            fish_type.woodland_fish_image,
            fish_type.s3_fish_image,
            fish_type.s3_woodland_image,
            fish.mercury,
            fish.omega_3,
            fish.pcb,
            fish.protein
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
