use crate::authentication::{validate_credentials, AuthError, Credentials};
use actix_web::{error::InternalError, web, HttpResponse};
use chrono::Utc;
use secrecy::Secret;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    user_id: Uuid,
    is_admin: bool,
    data: UserData,
}

#[derive(serde::Serialize)]
pub struct UserData {
    weight: Option<i16>,
    age: Option<i16>,
    sex: Option<String>,
    plan_to_get_pregnant: Option<bool>,
    portion_size: Option<i16>,
    image_url: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

/// Checks that the provided credentials are correct.
#[tracing::instrument(
    name="Logging in a user",
    skip(form, db_pool),
    fields(
        subscriber_name = %form.email
        )
    )]
pub async fn login(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        email: form.0.email,
        password: form.0.password,
    };
    tracing::Span::current().record("username", tracing::field::display(&credentials.email));
    match validate_credentials(credentials, &db_pool).await {
        Ok(user_data) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_data.0));
            match get_user_db(&db_pool, user_data.0).await {
                Ok(data) => Ok(HttpResponse::Ok().json(LoginResponse {
                    user_id: user_data.0,
                    is_admin: user_data.1,
                    data,
                })),
                // In the error case still allow the user to login.
                Err(e) => {
                    tracing::error!("User was validated, but unable to retrieve their data: {e}");
                    Ok(HttpResponse::Ok().json(user_data))
                }
            }
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            let response = HttpResponse::Unauthorized().finish();
            Err(InternalError::from_response(e, response))
        }
    }
}

#[tracing::instrument(name = "Getting user details from the db.", skip(db_pool, user_id))]
async fn get_user_db(db_pool: &PgPool, user_id: Uuid) -> Result<UserData, sqlx::Error> {
    let user_data = sqlx::query_as!(
        UserData,
        r#"
        SELECT
            weight,
            age,
            sex,
            plan_to_get_pregnant,
            portion_size,
            image_url,
            first_name,
            last_name
        FROM users
        WHERE id=$1;
        "#,
        user_id
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query: {:?}", e);
        e
    })?;

    sqlx::query!(
        r#"
        UPDATE users
        SET latest_login=$2
        WHERE id=$1
        "#,
        user_id,
        Utc::now()
    )
    .execute(db_pool)
    .await?;

    Ok(user_data)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
