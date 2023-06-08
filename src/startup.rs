use crate::middleware::{api_auth, reject_non_admin_users};
use crate::routes;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_lab::middleware::from_fn;
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
            .route("/health_check", web::get().to(routes::health_check))
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .wrap(TracingLogger::default())
                    .service(routes::fish_route)
                    .service(routes::fishs_route)
                    .service(routes::fish_avg_route)
                    .service(routes::fish_avgs)
                    .service(routes::recipe)
                    .service(routes::recipes)
                    .service(routes::min_and_max)
                    .service(routes::everything)
                    .service(routes::presign_s3)
                    .route("/search", web::get().to(routes::search))
                    .route("/register", web::post().to(routes::register))
                    .route("/login", web::post().to(routes::login))
                    .route("/user/profile", web::post().to(routes::update_profile))
                    .route("/user/image", web::post().to(routes::update_image))
                    .route(
                        "/user/change_password",
                        web::post().to(routes::change_password),
                    )
                    .route("/user/account", web::post().to(routes::update_account))
                    .route("/user/delete/{uuid}", web::post().to(routes::delete_user))
                    .service(
                        web::scope("/admin")
                            .wrap(from_fn(reject_non_admin_users))
                            .service(routes::update_recipe)
                            .route("/", web::get().to(greet)),
                    ),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
