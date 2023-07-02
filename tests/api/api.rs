use crate::helpers::spawn_app;

#[tokio::test]
async fn users_need_an_api_key_use_the_api() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/fishs", &app.address))
        .send()
        .await
        .expect("Unable to get fish.");

    assert_eq!(response.status().as_u16(), 401);

    let response = app
        .api_client
        .get(format!("{}/v1/fish_avg", &app.address))
        .send()
        .await
        .expect("Unable to get fish.");

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn only_admin_users_can_post_admin_routes() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "name": "Test name",
        "steps": [
            "step"
        ],
        "ingredients": [
            "ingredient"
        ]
    });

    let response = app
        .api_client
        .post(format!("{}/v1/admin/recipe/", &app.address))
        .json(&body)
        .header(
            "Cookie",
            &format!("user_id={}", &app.test_user.id.to_string()),
        )
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to post new recipe with test user.");

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn only_admin_users_can_get_admin_routes() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!(
            "{}/v1/admin/fish_type/{}",
            app.address, app.fish_type.id
        ))
        .header(
            "Cookie",
            &format!("user_id={}", &app.test_user.id.to_string()),
        )
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to post new recipe with test user.");

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn a_user_id_cookie_is_required_for_admin_routes() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!(
            "{}/v1/admin/fish_type/{}",
            app.address, app.fish_type.id
        ))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to post new recipe with test user.");

    assert_eq!(response.status().as_u16(), 500);
}
