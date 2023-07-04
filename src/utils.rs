use actix_web::HttpRequest;
use uuid::Uuid;

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// Return an opaque 400 while preserving the error root's cause for logging.
pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

/// Gets the user id from the request and throws an error if it's not
/// included or is an invalid uuid.
pub fn get_user_id(req: HttpRequest) -> Result<Uuid, actix_web::Error> {
    let user_id = req
        .cookie("user_id")
        .ok_or(e500("no user_id cookie included with the request."))?
        .value()
        .to_owned();

    let user_id = parse_user_id(&user_id)?;

    Ok(user_id)
}

/// Gets the user id from the request if it's there, otherwise returns
/// a None.
pub fn get_optional_user_id(req: HttpRequest) -> Result<Option<Uuid>, actix_web::Error> {
    match req.cookie("user_id") {
        Some(user_id) => Ok(Some(parse_user_id(user_id.value())?)),
        None => Ok(None),
    }
}

fn parse_user_id(cookie_value: &str) -> Result<Uuid, actix_web::Error> {
    Uuid::parse_str(cookie_value).map_err(actix_web::error::ErrorBadRequest)
}
