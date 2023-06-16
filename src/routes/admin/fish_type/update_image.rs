use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FishTypeId {
    uuid: Uuid,
}

#[derive(serde::Deserialize)]
pub struct FishTypeImageData {
    image_url: String,
    woodland_image_flag: bool,
}

#[tracing::instrument(name = "Updating a fish image.", skip(data, db_pool))]
#[post("/{uuid}/image")]
pub async fn update_fish_type_image(
    fish_type_id: web::Path<FishTypeId>,
    data: web::Json<FishTypeImageData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    if data.woodland_image_flag {
        match update_woodland_fish_image_db(&db_pool, fish_type_id.uuid, data).await {
            Ok(_) => {
                tracing::info!("Woodland fish image has been updated.");
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                tracing::error!("Failed to execute query: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        match update_fish_image_db(&db_pool, fish_type_id.uuid, data).await {
            Ok(_) => {
                tracing::info!("Fish image has been updated.");
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                tracing::error!("Failed to execute query: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[tracing::instrument(name = "Saving new image url to db.", skip(db_pool, data))]
async fn update_fish_image_db(
    db_pool: &PgPool,
    fish_type_id: Uuid,
    data: web::Json<FishTypeImageData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE fish_type
        SET s3_fish_image = $1
        WHERE id = $2;
        "#,
        data.image_url,
        fish_type_id,
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(name = "Saving new woodland image url to db.", skip(db_pool, data))]
async fn update_woodland_fish_image_db(
    db_pool: &PgPool,
    fish_type_id: Uuid,
    data: web::Json<FishTypeImageData>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE fish_type
        SET s3_woodland_image = $1
        WHERE id = $2;
        "#,
        data.image_url,
        fish_type_id,
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
