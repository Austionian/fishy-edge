use actix_web::{web, HttpResponse};

pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Data<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
