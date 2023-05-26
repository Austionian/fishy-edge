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

impl ToString for Sex {
    fn to_string(&self) -> String {
        match self {
            Sex::Female => "Female".to_string(),
            Sex::Male => "Male".to_string(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: Uuid,
    weight: i16,
    age: i16,
    sex: Sex,
    plan_to_get_pregnant: Option<bool>,
    portion_size: i16,
}

/// Adds a new user to the database and returns a 200 OK response on success.
/// Expects the user's email and password to be included in the form data.
#[tracing::instrument(
    name="Saving user's info",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.user_id
        )
    )]
pub async fn post_user(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match update_user(&db_pool, form).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e500(e)),
    }
}

#[tracing::instrument(name = "Saving user details to the db.", skip(db_pool, form))]
async fn update_user(db_pool: &PgPool, form: web::Form<FormData>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET (weight, age, sex, plan_to_get_pregnant, portion_size) = ($1, $2, $3, $4, $5)
        WHERE users.id = $6
        "#,
        form.weight,
        form.age,
        form.sex.to_string(),
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
