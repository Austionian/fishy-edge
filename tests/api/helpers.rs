use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use fishy_edge::configuration::{get_configuration, DataBaseSettings};
use fishy_edge::startup::run;
use fishy_edge::telemetry;
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub test_user: TestUser,
    pub admin_user: AdminUser,
    pub api_client: reqwest::Client,
    pub api_key: &'static str,
}

impl TestApp {
    // pub async fn login<Body>(&self, body: &Body) -> reqwest::Response
    // where
    //     Body: serde::Serialize,
    // {
    //     self.api_client
    //         .post(&format!("{}/login", &self.address))
    //         .form(body)
    //         .send()
    //         .await
    //         .expect("Failed to login.")
    // }
    pub async fn post_to_admin_with_non_admin_user<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/v1/admin/recipe/", &self.address))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.test_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to post new recipe with test user.")
    }

    pub async fn post_new_recipe<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/v1/admin/recipe/", &self.address))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to post new recipe.")
    }

    pub async fn update_recipe<Body>(&self, body: &Body, recipe_id: &str) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/v1/admin/recipe/{}", &self.address, recipe_id))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to post recipe update.")
    }

    pub async fn delete_recipe(&self, recipe_id: &str) -> reqwest::Response {
        self.api_client
            .post(format!(
                "{}/v1/admin/recipe/delete/{}",
                &self.address, recipe_id
            ))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to post recipe delete.")
    }

    pub async fn post_new_fish_type<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/v1/admin/fish_type/", &self.address))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to post new fish type.")
    }

    pub async fn update_fish_type<Body>(&self, body: &Body, fish_type_id: &str) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!(
                "{}/v1/admin/fish_type/{}",
                &self.address, fish_type_id
            ))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to update fish type.")
    }

    pub async fn post_new_fish<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/v1/admin/fish/", &self.address))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to post new fish.")
    }

    pub async fn update_fish<Body>(&self, body: &Body, fish_id: &str) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/v1/admin/fish/{}", &self.address, fish_id))
            .json(body)
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to update fish.")
    }

    pub async fn delete_fish(&self, fish_id: &str) -> reqwest::Response {
        self.api_client
            .post(format!(
                "{}/v1/admin/fish/delete/{}",
                &self.address, fish_id
            ))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to update fish.")
    }
}

async fn configure_database(config: &DataBaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, &config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    // ensure that trancing is only initialized on the first call
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address.");

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let _ = tokio::spawn(server);

    let test_app = TestApp {
        address,
        db_pool: connection_pool,
        api_client: client,
        test_user: TestUser::new(),
        admin_user: AdminUser::new(),
        api_key: "1234567890",
    };

    test_app.test_user.store(&test_app.db_pool).await;
    test_app.admin_user.store(&test_app.db_pool).await;

    test_app
}

pub struct TestUser {
    user_id: Uuid,
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn new() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            email: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        sqlx::query!(
            "INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3);",
            self.user_id,
            self.email,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }
}

pub struct AdminUser {
    user_id: Uuid,
    pub email: String,
    pub password: String,
}

impl AdminUser {
    pub fn new() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            email: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        sqlx::query!(
            "INSERT INTO users (id, email, password_hash, is_admin)
            VALUES ($1, $2, $3, $4);",
            self.user_id,
            self.email,
            password_hash,
            true
        )
        .execute(pool)
        .await
        .expect("Failed to store admin user.");
    }
}
