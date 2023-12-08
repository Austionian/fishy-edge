use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn admin_users_can_crud_recipes() {
    // Part One: Create a new recipe.
    let app = spawn_app().await;
    let name = Uuid::new_v4();
    let image_url = "https://fake_url.com";
    let body = serde_json::json!({
        "name": name,
        "image_url": image_url,
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
    assert_eq!(recipe.image_url.unwrap(), image_url);

    let new_image_url = "https:://new_fake_url.com";

    // Part Two: Update the recipe
    let body = serde_json::json!({
        "name": name,
        "image_url": new_image_url,
        "steps": [
            "step",
            "step2"
        ],
        "ingredients": []
    });

    let response = app.update_recipe(&body, &recipe.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let recipe = sqlx::query!("SELECT * FROM recipe WHERE id = $1", recipe.id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get the updated recipe.");

    assert_eq!(recipe.name, name.to_string());
    assert_eq!(recipe.steps.unwrap().len(), 2);
    assert_eq!(recipe.ingredients.unwrap().len(), 0);
    assert_eq!(recipe.image_url.unwrap(), new_image_url);

    // Part Three: Delete the recipe
    let response = app.delete_recipe(&recipe.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let recipes = sqlx::query!("SELECT * FROM recipe WHERE id = $1", recipe.id)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to execute the recipe select.");

    assert_eq!(recipes.len(), 0);
}

#[tokio::test]
async fn admin_users_can_crud_fish_types() {
    // Part One: Create new fish type and a fish instance of the type
    let app = spawn_app().await;
    let name = Uuid::new_v4();
    let body = serde_json::json!({
        "name": name,
        "anishinaabe_name": name,
        "fish_image": "path_to_image",
        "about": "This is a new fish type added for tests."
    });

    let response = app.post_new_fish_type(&body).await;

    assert_eq!(response.status().as_u16(), 200);

    let fish_type = sqlx::query!("SELECT * FROM fish_type WHERE name = $1", name.to_string())
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get fish type from db.");

    assert_eq!(fish_type.name, name.to_string());
    assert_eq!(fish_type.anishinaabe_name.unwrap(), name.to_string());
    assert_eq!(fish_type.s3_fish_image.unwrap(), "path_to_image");
    assert_eq!(fish_type.fish_image, None);
    assert_eq!(fish_type.about, "This is a new fish type added for tests.");

    // Part Two: Update fish type
    let body = serde_json::json!({
        "name": name,
        "anishinaabe_name": "anishinaabe name test",
        "about": "This is the new about text."
    });

    let response = app.update_fish_type(&body, &fish_type.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let fish_type = sqlx::query!("SELECT * FROM fish_type WHERE name = $1", name.to_string())
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to get fish type from db.");

    assert_eq!(fish_type.name, name.to_string());
    assert_eq!(fish_type.anishinaabe_name.unwrap(), "anishinaabe name test");
    assert_eq!(fish_type.about, "This is the new about text.");

    // Part Three: Read the new fish type.
    let response = app.get_fish_type(fish_type.id.to_string().as_str()).await;

    assert_eq!(response.status().as_u16(), 200);

    // Part Four: Read all the fish types.
    let response = app.get_all_fish_types().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn admin_users_can_crud_fish() {
    // Part One: Create the fish instance
    let app = spawn_app().await;

    let lake = "Michigan";
    let body = serde_json::json!({
        "fish_type_id": &app.fish_type.id,
        "lake": lake,
        "mercury": 1.1,
        "omega_3": 1.1,
        "omega_3_ratio": 1.1,
        "pcb": 1.1,
        "protein": 1.1
    });

    let response = app.post_new_fish(&body).await;

    assert_eq!(response.status().as_u16(), 200);

    let fish = sqlx::query!(
        "SELECT * FROM fish WHERE fish_type_id = $1 AND lake = $2",
        &app.fish_type.id,
        lake
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to get the created fish.");

    assert_eq!(fish.mercury.unwrap(), 1.1);
    assert_eq!(fish.omega_3.unwrap(), 1.1);

    // Part Two: Update the fish instance
    let body = serde_json::json!({
        "fish_type_id": &app.fish_type.id,
        "lake": lake,
        "mercury": 2.1,
        "omega_3": 2.1,
        "omega_3_ratio": 2.1,
        "pcb": 2.1,
        "protein": 2.1
    });

    let response = app.update_fish(&body, &fish.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let fish = sqlx::query!(
        "SELECT * FROM fish WHERE fish_type_id = $1 AND lake = $2",
        &app.fish_type.id,
        lake
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to get the created fish.");

    assert_eq!(fish.mercury.unwrap(), 2.1);
    assert_eq!(fish.omega_3.unwrap(), 2.1);

    // Part Three: Delete the fish instance
    let response = app.delete_fish(&fish.id.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);

    let fishs = sqlx::query!("SELECT * FROM fish WHERE id = $1", fish.id)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to get fishs.");

    assert_eq!(fishs.len(), 0);
}

#[tokio::test]
async fn admins_should_be_able_to_read_fish_types() {
    let app = spawn_app().await;

    let response = app
        .get_fish_type(&app.fish_type.id.to_string().as_str())
        .await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn fish_type_read_route_should_return_bad_request_for_made_up_uuid() {
    let app = spawn_app().await;

    let response = app
        .get_fish_type(uuid::Uuid::new_v4().to_string().as_str())
        .await;

    assert_eq!(response.status().as_u16(), 400);
}
