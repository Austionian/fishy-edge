use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute the request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = fishy_edge::run(listener).expect("Failed to bind address.");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let body = "name=austin%20rooks&email=austin%40r00ks.io";
    let response = client
        .post(&format!("{}/subscribe", &app_address))
        .body(body)
        .send()
        .await
        .expect("Failed to execute the requst.");

    assert_eq!(200, response.status().as_u16())
}
