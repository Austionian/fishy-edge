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
        &app.test_user.user_id,
    );

    let response = app.update_profile(body).await;

    assert_eq!(response.status().as_u16(), 200);

    let user = sqlx::query!("SELECT * FROM users WHERE id = $1;", &app.test_user.user_id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get the user from the db.");

    assert_eq!(user.weight, Some(weight));
    assert_eq!(user.age, Some(age));
    assert_eq!(user.sex, Some(sex.to_string()));
    assert_eq!(user.plan_to_get_pregnant, None);
    assert_eq!(user.portion_size, Some(portion_size));
}
