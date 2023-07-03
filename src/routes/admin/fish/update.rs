use actix_web::{put, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FishUuid {
    uuid: Uuid,
}

#[derive(serde::Deserialize)]
pub struct FishData {
    pub(crate) mercury: f32,
    pub(crate) omega_3: f32,
    pub(crate) omega_3_ratio: f32,
    pub(crate) pcb: f32,
    pub(crate) protein: f32,
}

#[tracing::instrument(name = "Updating fish data", skip(uuid, data, db_pool))]
#[put("/{uuid}")]
pub async fn update_fish(
    uuid: web::Path<FishUuid>,
    data: web::Json<FishData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match update_fish_db(&db_pool, uuid.uuid, data).await {
        Ok(_) => {
            tracing::info!("Fish has been updated.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving fish data to the database", skip(db_pool, data))]
async fn update_fish_db(
    db_pool: &PgPool,
    fish_uuid: Uuid,
    data: web::Json<FishData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE fish
        SET 
            mercury = $1,
            omega_3 = $2,
            omega_3_ratio = $3,
            pcb = $4,
            protein = $5
        WHERE id = $6
        "#,
        data.mercury,
        data.omega_3,
        data.omega_3_ratio,
        data.pcb,
        data.protein,
        fish_uuid,
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
