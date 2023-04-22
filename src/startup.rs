use crate::middleware::api_auth;
use crate::routes::{all_fish, fish, fishs, health_check, recipe, recipes, register, search};
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

        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/hello/{name}", web::get().to(greet))
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .wrap(TracingLogger::default())
                    .service(fishs)
                    .service(fish)
                    .service(recipe)
                    .service(recipes)
                    .service(all_fish)
                    .route("/search", web::get().to(search))
                    .route("/register", web::post().to(register)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
