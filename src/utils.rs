use actix_web::HttpRequest;
use uuid::Uuid;

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

/// Get the user id from the cookie in the HTTP request.
pub fn get_user_id(req: HttpRequest) -> Result<Uuid, actix_web::Error> {
    let user_id = Uuid::parse_str(
        req.cookie("user_id")
            .ok_or(e500("no user_id cookie included with the request."))?
            .value(),
    )
    .map_err(actix_web::error::ErrorBadRequest)?;

    Ok(user_id)
}
