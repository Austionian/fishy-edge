use crate::helpers::spawn_app;

#[tokio::test]
async fn a_user_should_be_able_get_recipes() {
    let app = spawn_app().await;

    let response = app.get_recipes().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn a_user_should_not_need_to_provide_a_user_id_to_see_recipes() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/recipe/", &app.address))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to post unfavorite recipe.");

    assert_eq!(response.status().as_u16(), 200);
}
