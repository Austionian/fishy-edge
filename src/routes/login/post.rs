use crate::authentication::{validate_credentials, AuthError, Credentials};
use actix_web::{error::InternalError, web, HttpResponse};
use secrecy::Secret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
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
    tracing::Span::current().record("username", &tracing::field::display(&credentials.email));
    match validate_credentials(credentials, &db_pool).await {
        Ok(user_data) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_data.0));
            Ok(HttpResponse::Ok().json(user_data))
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
