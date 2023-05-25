use crate::helpers::spawn_app;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;

#[tokio::test]
async fn register_returns_a_200() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let email: String = SafeEmail().fake();

    let body = format!("email={email}&password=test");
    let response = client
        .post(&format!("{}/v1/register", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", "Bearer 1234567890")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let user_id = response.json::<uuid::Uuid>().await.unwrap();

    let saved = sqlx::query!("SELECT email From users WHERE id = $1", user_id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.email, email);
}

#[tokio::test]
async fn subscribe_returns_a_400_with_incomplete_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = "";
    let response = client
        .post(format!("{}/v1/register", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", "Bearer 1234567890")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request.");

    assert_eq!(response.status().as_u16(), 400);
}
