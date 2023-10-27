use crate::middleware::{api_auth, reject_non_admin_users};
use crate::routes;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_lab::middleware::from_fn;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

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
                    .service(routes::fish)
                    .service(routes::fishs)
                    .service(routes::fish_avg)
                    .service(routes::fish_avgs)
                    .service(routes::recipe)
                    .service(routes::recipes)
                    .service(routes::min_and_max)
                    .service(routes::everything)
                    .service(routes::presign_s3)
                    .service(
                        web::scope("/favorite")
                            .service(routes::favorites)
                            .service(routes::favorite_fish)
                            .service(routes::favorite_recipe),
                    )
                    .service(
                        web::scope("/unfavorite")
                            .service(routes::unfavorite_fish)
                            .service(routes::unfavorite_recipe),
                    )
                    .route("/search", web::get().to(routes::search))
                    .route("/register", web::post().to(routes::register))
                    .route("/login", web::post().to(routes::login))
                    .service(
                        web::scope("/user")
                            .route("/profile", web::post().to(routes::update_profile))
                            .route("/account", web::post().to(routes::update_account))
                            .route("/image", web::post().to(routes::update_image))
                            .route("/change_password", web::post().to(routes::change_password))
                            .route("/{uuid}", web::delete().to(routes::delete_user)),
                    )
                    .service(
                        web::scope("/admin")
                            .wrap(from_fn(reject_non_admin_users))
                            .service(
                                web::scope("/recipe")
                                    .service(routes::new_recipe)
                                    .service(routes::update_recipe)
                                    .service(routes::delete_recipe)
                                    .service(routes::update_recipe_image),
                            )
                            .service(
                                web::scope("/fish")
                                    .service(routes::new_fish)
                                    .service(routes::update_fish)
                                    .service(routes::delete_fish),
                            )
                            .service(
                                web::scope("/fish_type")
                                    .service(routes::create_fish_type)
                                    .service(routes::update_fish_type)
                                    .service(routes::read_fish_type)
                                    .service(routes::read_all_fish_types)
                                    .service(routes::update_fish_type_image),
                            )
                            .service(web::scope("/analytics").service(routes::get_analytics)),
                    ),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
