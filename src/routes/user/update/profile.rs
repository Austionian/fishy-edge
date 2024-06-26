use std::fmt::Display;

use crate::utils::e500;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "sex")]
#[sqlx(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
}

impl Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Female => write!(f, "Female"),
            Sex::Male => write!(f, "Male"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: Uuid,
    weight: i16,
    age: i16,
    plan_to_get_pregnant: Option<bool>,
    portion_size: i16,
}

/// Updates a user's profile information.
#[tracing::instrument(
    name="Saving user's info",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.user_id
        )
    )]
pub async fn update_profile(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match update_profile_db(&db_pool, form).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e500(e)),
    }
}

#[tracing::instrument(name = "Saving user details to the db.", skip(db_pool, form))]
async fn update_profile_db(db_pool: &PgPool, form: web::Form<FormData>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET (weight, age, sex, plan_to_get_pregnant, portion_size) = ($1, $2, $3, $4, $5)
        WHERE users.id = $6
        "#,
        form.weight,
        form.age,
        "male",
        form.plan_to_get_pregnant,
        form.portion_size,
        form.user_id,
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    Ok(())
}
