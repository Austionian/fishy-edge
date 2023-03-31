use crate::helpers::spawn_app;

#[tokio::test]
async fn requires_an_api_key() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/v1/fishs", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn fishs_gets_fish_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // populate the db and then assert below.

    let response = client
        .get(&format!("{}/v1/fishs", &app.address))
        .header("Authorization", "Bearer 1234567890")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}
