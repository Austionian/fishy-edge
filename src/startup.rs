use crate::middleware::api_auth;
use crate::routes;
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
            .route("/health_check", web::get().to(routes::health_check))
            .route("/hello/{name}", web::get().to(greet))
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .wrap(TracingLogger::default())
                    .service(routes::fish)
                    .service(routes::fishs)
                    .service(routes::fish_avg)
                    .service(routes::fish_avgs)
                    .service(routes::recipe)
                    .service(routes::recipes)
                    .service(routes::min_and_max)
                    .service(routes::everything)
                    .service(routes::presign_s3)
                    .route("/search", web::get().to(routes::search))
                    .route("/register", web::post().to(routes::register)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
