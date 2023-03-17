use fishy_edge::configuration::get_configuration;
use fishy_edge::startup::run;
use fishy_edge::telemetry;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("fishy_edge".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // Get config and connect to Postgres
    let config = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;

    Ok(())
}
