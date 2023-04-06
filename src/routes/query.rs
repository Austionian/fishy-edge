use actix_web::HttpResponse;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct AllFishData {
    id: Uuid,
    fish_id: Uuid,
    name: String,
    anishinaabe_name: Option<String>,
    fish_image: Option<String>,
    woodland_fish_image: Option<String>,
    s3_fish_image: Option<String>,
    s3_woodland_image: Option<String>,
}

#[tracing::instrument(name = "Querying")]
pub async fn query() -> HttpResponse {
    HttpResponse::Ok().finish()
}
