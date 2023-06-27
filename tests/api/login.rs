use crate::helpers::spawn_app;

#[tokio::test]
async fn a_user_should_be_able_to_login() {
    let app = spawn_app().await;

    let body = format!(
        "email={}&password={}",
        &app.test_user.email, &app.test_user.password_hash
    );

    let response = app.login(body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn the_password_needs_to_be_correct() {
    let app = spawn_app().await;

    let body = format!("email={}&password=somemadeuppassword", &app.test_user.email);

    let response = app.login(body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn an_email_needs_to_be_included_in_the_request() {
    let app = spawn_app().await;

    let body = format!("password={}", &app.test_user.password_hash);

    let response = app.login(body).await;

    assert_eq!(response.status().as_u16(), 400);
}
