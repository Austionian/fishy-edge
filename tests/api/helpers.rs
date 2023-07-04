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
    pub fish_type: FishType,
    pub fish: Fish,
    pub api_client: reqwest::Client,
    pub api_key: &'static str,
}

impl TestApp {
    pub async fn get_test_user_from_db(&self) -> Result<TestUser, sqlx::Error> {
        sqlx::query_as!(
            TestUser,
            "SELECT * FROM users WHERE id = $1;",
            &self.test_user.id
        )
        .fetch_one(&self.db_pool)
        .await
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
            .put(format!("{}/v1/admin/recipe/{}", &self.address, recipe_id))
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
            .delete(format!("{}/v1/admin/recipe/{}", &self.address, recipe_id))
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

    pub async fn get_all_fish_types(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/admin/fish_type/", &self.address))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to get all fish types.")
    }

    pub async fn get_fish_type(&self, fish_type_id: &str) -> reqwest::Response {
        self.api_client
            .get(format!(
                "{}/v1/admin/fish_type/{}",
                &self.address, fish_type_id
            ))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to get fish type.")
    }

    pub async fn update_fish_type<Body>(&self, body: &Body, fish_type_id: &str) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .put(&format!(
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
            .put(format!("{}/v1/admin/fish/{}", &self.address, fish_id))
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
            .delete(format!("{}/v1/admin/fish/{}", &self.address, fish_id))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to update fish.")
    }

    pub async fn update_profile(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/user/profile", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .body(body)
            .send()
            .await
            .expect("Failed to update profile.")
    }

    pub async fn update_account(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/user/account", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .body(body)
            .send()
            .await
            .expect("Failed to update account.")
    }

    pub async fn update_image(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/user/image", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .body(body)
            .send()
            .await
            .expect("Failed to update account.")
    }

    pub async fn change_password(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/user/change_password", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .body(body)
            .send()
            .await
            .expect("Failed to change password.")
    }

    pub async fn login(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/v1/login", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .body(body)
            .send()
            .await
            .expect("Failed to login.")
    }

    pub async fn delete_account(&self) -> reqwest::Response {
        self.api_client
            .delete(format!("{}/v1/user/{}", &self.address, &self.test_user.id))
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to delete account.")
    }

    pub async fn get_search(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/search", &self.address))
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to get search.")
    }

    pub async fn get_fish_by_id(&self, fish_id: Uuid) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/fish/{}", &self.address, fish_id))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to get fish.")
    }

    pub async fn get_fish_type_avg(&self, fish_type_id: &Uuid) -> reqwest::Response {
        self.api_client
            .get(format!(
                "{}/v1/fish_avg?fishtype_id={}",
                &self.address, fish_type_id
            ))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to get fish type avg.")
    }

    pub async fn get_favorites(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/v1/favorite/", &self.address))
            .header(
                "Cookie",
                &format!("user_id={}", &self.admin_user.user_id.to_string()),
            )
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .send()
            .await
            .expect("Failed to get favorites.")
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

    let fish_type_id = Uuid::new_v4();

    let test_app = TestApp {
        address,
        db_pool: connection_pool,
        api_client: client,
        test_user: TestUser::new(),
        admin_user: AdminUser::new(),
        fish_type: FishType::new(fish_type_id),
        fish: Fish::new(fish_type_id),
        api_key: "1234567890",
    };

    test_app.test_user.store(&test_app.db_pool).await;
    test_app.admin_user.store(&test_app.db_pool).await;
    test_app.fish_type.store(&test_app.db_pool).await;
    test_app.fish.store(&test_app.db_pool).await;

    test_app
}

pub struct TestUser {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i16>,
    pub weight: Option<i16>,
    pub sex: Option<String>,
    pub plan_to_get_pregnant: Option<bool>,
    pub portion_size: Option<i16>,
    pub is_admin: Option<bool>,
    pub image_url: Option<String>,
}

impl TestUser {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            email: Uuid::new_v4().to_string(),
            name: None,
            password_hash: Uuid::new_v4().to_string(),
            first_name: None,
            last_name: None,
            age: None,
            weight: None,
            sex: None,
            plan_to_get_pregnant: None,
            portion_size: None,
            is_admin: Some(false),
            image_url: None,
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password_hash.as_bytes(), &salt)
        .unwrap()
        .to_string();

        sqlx::query!(
            "INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3);",
            self.id,
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

pub struct FishType {
    pub id: Uuid,
    pub name: &'static str,
    pub anishinaabe_name: &'static str,
    pub about: &'static str,
}

impl FishType {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            name: "Test Fish",
            anishinaabe_name: "Test Anishaabe Name",
            about: "About a test fish.",
        }
    }

    pub async fn store(&self, db_pool: &PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO fish_type (id, name, anishinaabe_name, about)
            VALUES ($1, $2, $3, $4);
            "#,
            &self.id,
            &self.name,
            &self.anishinaabe_name,
            &self.about
        )
        .execute(db_pool)
        .await
        .expect("Failed to store fish type.");
    }
}

#[derive(serde::Deserialize)]
pub struct Fish {
    pub id: Uuid,
    pub fish_type_id: Uuid,
    pub lake: String,
    pub mercury: Option<f32>,
    pub omega_3: Option<f32>,
    pub omega_3_ratio: Option<f32>,
    pub pcb: Option<f32>,
    pub protein: Option<f32>,
}

impl Fish {
    pub fn new(fish_type_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            fish_type_id,
            lake: "Lake".to_string(),
            mercury: Some(1.12),
            omega_3: Some(1.12),
            omega_3_ratio: Some(1.12),
            pcb: Some(1.12),
            protein: Some(1.12),
        }
    }

    pub async fn store(&self, db_pool: &PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO fish (id, fish_type_id, lake, mercury, omega_3, omega_3_ratio, pcb, protein)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            &self.id,
            &self.fish_type_id,
            &self.lake,
            &self.mercury.unwrap(),
            &self.omega_3.unwrap(),
            &self.omega_3_ratio.unwrap(),
            &self.pcb.unwrap(),
            &self.protein.unwrap(),
        )
        .execute(db_pool)
        .await
        .expect("Failed to store fish.");
    }
}
