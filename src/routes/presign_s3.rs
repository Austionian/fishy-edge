use crate::configuration;
use actix_web::{post, web, HttpResponse};
use s3::bucket::Bucket;
use s3::creds::Credentials;

#[derive(serde::Deserialize)]
pub struct File {
    name: String,
}

#[derive(serde::Serialize)]
pub struct Presign {
    url: String,
}

/// Adds a new user to the database and returns a 200 OK response on success.
/// Expects the user's email to be included in the form data.
#[tracing::instrument(name = "Generating a presigned URL", skip(file))]
#[post("/presign_s3")]
pub async fn presign_s3(file: web::Json<File>) -> HttpResponse {
    let config = configuration::get_configuration().expect("Failed to get configuration.");
    let bucket_name = config.s3.bucket;
    let region = config
        .s3
        .region
        .parse()
        .expect("Provided region was invalid.");
    let credentials = Credentials::new(
        Some(&config.s3.access_key_id),
        Some(&config.s3.secret_access_key),
        None,
        None,
        None,
    )
    .expect("Failed to create credentials.");
    let bucket = Bucket::new(&bucket_name, region, credentials)
        .expect("Failed to create a bucket instance.");

    let url = bucket
        .presign_put(&file.name, 86400, None)
        .map_err(|e| {
            tracing::error!("Failed to generate the presigned url: {:?}", e);
            e
        })
        .unwrap();

    let url = Presign { url };

    // Save to db with filename

    HttpResponse::Ok().json(url)
}
