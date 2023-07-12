use crate::helpers::spawn_app;

#[tokio::test]
async fn you_shoud_be_able_to_see_your_favorite_fish_and_recipes() {
    let app = spawn_app().await;

    let response = app.get_favorites().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn a_user_id_is_required_to_see_favorites() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/favorite/", &app.address))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to get favorites.");

    assert_eq!(response.status().as_u16(), 500);
}

#[tokio::test]
async fn a_valid_uuid_user_id_is_required_to_see_favorites() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/favorite/", &app.address))
        .header("Cookie", &format!("user_id={}", "not-a-uuid"))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to get favorites.");

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn a_user_can_favorite_and_unfavorite_a_fish() {
    let app = spawn_app().await;

    // Part One: Favorite the fish.
    let response = app.favorite_fish(&app.fish_type.id).await;

    assert_eq!(response.status().as_u16(), 200);

    let favorite = sqlx::query!(
        "SElECT * FROM user_fishtype WHERE user_id = $1",
        &app.test_user.id
    )
    .fetch_all(&app.db_pool)
    .await
    .expect("Failed to find a favorite for user.");

    assert_eq!(favorite.len(), 1);

    // Part two: Unfavorite the fish.
    let response = app.unfavorite_fish(&app.fish_type.id).await;

    assert_eq!(response.status().as_u16(), 200);

    let favorite = sqlx::query!(
        "SElECT * FROM user_fishtype WHERE user_id = $1",
        &app.test_user.id
    )
    .fetch_all(&app.db_pool)
    .await
    .expect("Failed to find a favorite for user.");

    assert_eq!(favorite.len(), 0);
}

#[tokio::test]
async fn a_user_can_favorite_and_unfavorite_a_recipe() {
    let app = spawn_app().await;

    // Part One: Favorite the recipe.
    let response = app.favorite_recipe(&app.recipe.id).await;

    assert_eq!(response.status().as_u16(), 200);

    let favorite = sqlx::query!(
        "SElECT * FROM user_recipe WHERE user_id = $1",
        &app.test_user.id
    )
    .fetch_all(&app.db_pool)
    .await
    .expect("Failed to find a favorite recipe for user.");

    assert_eq!(favorite.len(), 1);

    // Part Two: Unfavorite the recipe.
    let response = app.unfavorite_recipe(&app.recipe.id).await;

    assert_eq!(response.status().as_u16(), 200);

    let favorite = sqlx::query!(
        "SElECT * FROM user_recipe WHERE user_id = $1",
        &app.test_user.id
    )
    .fetch_all(&app.db_pool)
    .await
    .expect("Failed to find a favorite recipe for user.");

    assert_eq!(favorite.len(), 0);
}
