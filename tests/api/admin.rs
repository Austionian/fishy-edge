use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn only_admin_users_can_hit_admin_routes() {
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

    let response = app.post_to_admin_with_non_admin_user(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn admin_users_can_crud_recipes() {
    // Part One: Create a new recipe.
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

    let response = app.post_new_recipe(&body).await;

    assert_eq!(response.status().as_u16(), 200);

    let recipe = sqlx::query!("SELECT * FROM recipe WHERE name = $1", name.to_string())
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get the created recipe.");

    assert_eq!(recipe.name, name.to_string());
    assert_eq!(recipe.steps.unwrap().len(), 1);
    assert_eq!(recipe.ingredients.unwrap().len(), 1);

    // Part Rwo: Update the recipe
    let body = serde_json::json!({
        "name": name,
        "steps": [
            "step",
            "step2"
        ],
        "ingredients": []
    });

    let response = app.update_recipe(&body, recipe.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let recipe = sqlx::query!("SELECT * FROM recipe WHERE id = $1", recipe.id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get the updated recipe.");

    assert_eq!(recipe.name, name.to_string());
    assert_eq!(recipe.steps.unwrap().len(), 2);
    assert_eq!(recipe.ingredients.unwrap().len(), 0);

    // Part Three: Delete the recipe
    let response = app.delete_recipe(recipe.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let recipes = sqlx::query!("SELECT * FROM recipe WHERE id = $1", recipe.id)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to execute the recipe select.");

    assert_eq!(recipes.len(), 0);
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

#[tokio::test]
async fn admin_users_can_create_new_fish() {
    let app = spawn_app().await;
    let fish_name = Uuid::new_v4();
    let body = serde_json::json!({
        "name": fish_name,
        "anishinaabe_name": fish_name,
        "fish_image": "path_to_image",
        "about": "This is a new fish type added for tests"
    });

    let response = app.post_new_fish_type(&body).await;

    assert_eq!(response.status().as_u16(), 200);

    let fish_type = sqlx::query!(
        "SELECT id FROM fish_type WHERE name = $1",
        fish_name.to_string()
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to get the created fish type.");

    let body = serde_json::json!({
        "fish_type_id": fish_type.id.to_string(),
        "lake": "Michigan",
        "mercury": 1.1,
        "omega_3": 1.1,
        "omega_3_ratio": 1.1,
        "pcb": 1.1,
        "protein": 1.1
    });

    let response = app.post_new_fish(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
