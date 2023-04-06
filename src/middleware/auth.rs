use crate::configuration::get_configuration;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

pub async fn api_auth(
    req: ServiceRequest,
    auth: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match validate_token(auth.token()) {
        Ok(_) => Ok(req),
        Err(_) => Err((AuthenticationError::from(Config::default()).into(), req)),
    }
}

fn validate_token(token: &str) -> Result<bool, std::io::Error> {
    let api_key = get_configuration().unwrap().application.api_key;
    if token.eq(&api_key) {
        return Ok(true);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Authenication failed.",
    ))
}

pub async fn api_auth_pub(
    req: ServiceRequest,
    auth: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match validate_public_token(auth.token()) {
        Ok(_) => Ok(req),
        Err(_) => Err((AuthenticationError::from(Config::default()).into(), req)),
    }
}

fn validate_public_token(token: &str) -> Result<bool, std::io::Error> {
    let public_key = get_configuration().unwrap().application.public_key;
    if token.eq(&public_key) {
        return Ok(true);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Authenication failed.",
    ))
}
