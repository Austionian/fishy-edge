use crate::helpers::spawn_app;

#[derive(serde::Deserialize)]
struct LoginResponseBody {
    user_id: uuid::Uuid,
    is_admin: bool,
}

#[tokio::test]
async fn a_user_should_be_able_to_change_their_password() {
    // Part One: Change the password
    let app = spawn_app().await;

    let new_password = uuid::Uuid::new_v4().to_string();

    let body = format!(
        "user_id={}&new_password={}&new_password_check={}",
        &app.test_user.id, new_password, new_password
    );

    let response = app.change_password(body).await;

    assert_eq!(response.status().as_u16(), 200);

    // Part Two: Check that the original password is no longer valid.
    let body = format!(
        "email={}&password={}",
        &app.test_user.email, &app.test_user.password_hash
    );

    let response = app.login(body).await;

    assert_eq!(response.status().as_u16(), 401);

    // Part Three: Check that the new password works
    let body = format!("email={}&password={}", &app.test_user.email, new_password);

    let response = app.login(body).await;

    assert_eq!(response.status().as_u16(), 200);

    let response_body: LoginResponseBody = response.json().await.unwrap();

    assert_eq!(response_body.is_admin, false);
    assert_eq!(response_body.user_id, app.test_user.id)
}

#[tokio::test]
async fn new_passwords_must_match() {
    let app = spawn_app().await;

    let new_password = uuid::Uuid::new_v4().to_string();
    let other_new_password = uuid::Uuid::new_v4().to_string();

    let body = format!(
        "user_id={}&new_password={}&new_password_check={}",
        &app.test_user.id, new_password, other_new_password
    );

    let response = app.change_password(body).await;

    assert_eq!(response.status().as_u16(), 400);
}

// #[tokio::test]
// async fn current_password_must_be_correct() {
//     let app = spawn_app().await;
//
//     let wrong_password = uuid::Uuid::new_v4().to_string();
//     let new_password = uuid::Uuid::new_v4().to_string();
//
//     let body = format!(
//         "user_id={}&new_password={}&new_password_check={}",
//         &app.test_user.id, wrong_password, new_password, new_password
//     );
//
//     let response = app.change_password(body).await;
//
//     assert_eq!(response.status().as_u16(), 400);
// }
