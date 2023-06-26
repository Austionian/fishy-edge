use crate::helpers::spawn_app;

#[tokio::test]
async fn a_user_should_be_able_to_update_their_profile() {
    let app = spawn_app().await;

    let weight = 200;
    let age = 23;
    let sex = "Male";
    let portion_size = 8;

    let body = format!(
        "user_id={}&weight={weight}&age={age}&sex={sex}&portion_size={portion_size}",
        &app.test_user.id,
    );

    let response = app.update_profile(body).await;

    assert_eq!(response.status().as_u16(), 200);

    let user = app.get_test_user_from_db().await;

    assert_eq!(user.weight, Some(weight));
    assert_eq!(user.age, Some(age));
    assert_eq!(user.sex, Some(sex.to_string()));
    assert_eq!(user.plan_to_get_pregnant, None);
    assert_eq!(user.portion_size, Some(portion_size));
}

#[tokio::test]
async fn a_user_should_be_able_to_update_their_account() {
    let app = spawn_app().await;
    let email = uuid::Uuid::new_v4();

    let body = format!(
        "user_id={}&email={}&first_name=&last_name=",
        &app.test_user.id, email
    );

    let response = app.update_account(body).await;

    assert_eq!(response.status().as_u16(), 200);

    let user = app.get_test_user_from_db().await;

    assert_eq!(user.email, email.to_string());
    assert_eq!(user.first_name, Some("".to_string()));
    assert_eq!(user.last_name, Some("".to_string()));
}

#[tokio::test]
async fn a_user_should_be_able_to_update_their_account_with_incomplete_data() {
    let app = spawn_app().await;
    let email = uuid::Uuid::new_v4();

    let body = format!("user_id={}&email={}&last_name=", &app.test_user.id, email);

    let response = app.update_account(body).await;

    assert_eq!(response.status().as_u16(), 200);

    let user = app.get_test_user_from_db().await;

    assert_eq!(user.email, email.to_string());
    assert_eq!(user.first_name, None);
    assert_eq!(user.last_name, Some("".to_string()));
}

#[tokio::test]
async fn a_user_should_be_able_to_update_their_profile_image() {
    let app = spawn_app().await;

    let body = format!(
        "user_id={}&image_url=http://test.url/test/path",
        &app.test_user.id
    );

    let response = app.update_image(body).await;

    assert_eq!(response.status().as_u16(), 200);

    let user = app.get_test_user_from_db().await;

    assert_eq!(
        user.image_url,
        Some("http://test.url/test/path".to_string())
    );
}

#[tokio::test]
async fn a_user_should_not_be_able_to_update_their_profile_image_with_incomplete_data() {
    let app = spawn_app().await;

    let body = format!("user_id={}", &app.test_user.id);

    let response = app.update_image(body).await;

    assert_eq!(response.status().as_u16(), 400);
}
