use fishy_edge::configuration::get_configuration;
use fishy_edge::startup::run;
use fishy_edge::telemetry;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("fishy_edge".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // Get config and connect to Postgres
    let config = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to the database.");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;

    Ok(())
}
