use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn only_admin_users_can_hit_admin_routes() {
    let app = spawn_app().await;

    let response = app
        .post_new_recipe(
            &serde_json::json!({
                "name": "recipe_name",
                "steps": [
                    "step"
                ],
                "ingredients": [
                    "ingredient"
                ]
            }),
            false,
        )
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn admin_users_can_create_new_recipes() {
    let app = spawn_app().await;
    let name = Uuid::new_v4();
    let body = serde_json::json!({
        "name": name,
        "steps": [
            "step"
        ],
        "ingredients": [
            "ingredient"
        ]
    });

    let response = app.post_new_recipe(&body, true).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn admin_users_can_create_new_fish_types() {
    let app = spawn_app().await;
    let name = Uuid::new_v4();
    let body = serde_json::json!({
        "name": name,
        "anishinaabe_name": name,
        "fish_image": "path_to_image",
        "about": "This is a new fish type added for tests"
    });

    let response = app.post_new_fish_type(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
