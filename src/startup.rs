use crate::middleware::{api_auth, api_auth_pub};
use crate::routes::{fish, fishs, health_check, query, register};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(api_auth);
        let pub_auth = HttpAuthentication::bearer(api_auth_pub);

        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/hello/{name}", web::get().to(greet))
            .service(
                web::scope("/public")
                    .wrap(pub_auth)
                    .wrap(TracingLogger::default())
                    .route("/", web::get().to(query)),
            )
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .wrap(TracingLogger::default())
                    .service(fishs)
                    .service(fish)
                    .route("/register", web::post().to(register)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
