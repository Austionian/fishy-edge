use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishData {
    pub(crate) fish_type_id: Uuid,
    pub(crate) lake: String,
    pub(crate) mercury: f32,
    pub(crate) omega_3: f32,
    pub(crate) omega_3_ratio: f32,
    pub(crate) pcb: f32,
    pub(crate) protein: f32,
}

#[tracing::instrument(name = "Creating a new fish.", skip(data, db_pool))]
#[post("/")]
pub async fn new_fish(data: web::Json<FishData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let fish_id = Uuid::new_v4();
    match new_fish_db(&db_pool, fish_id, data).await {
        Ok(_) => {
            tracing::info!("New fish has been added.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new fish data to the database", skip(db_pool, data))]
async fn new_fish_db(
    db_pool: &PgPool,
    fish_id: Uuid,
    data: web::Json<FishData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO fish (
            id,
            fish_type_id,
            lake,
            mercury,
            omega_3,
            omega_3_ratio,
            pcb,
            protein
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8
        );
        "#,
        fish_id,
        data.fish_type_id,
        data.lake,
        data.mercury,
        data.omega_3,
        data.omega_3_ratio,
        data.pcb,
        data.protein
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
