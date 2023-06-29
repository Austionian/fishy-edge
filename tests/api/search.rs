use crate::helpers::spawn_app;
use fishy_edge::routes::SearchResult;

#[tokio::test]
async fn search_should_return_everything_for_the_client() {
    let app = spawn_app().await;

    let response = app.get_search().await;

    assert_eq!(response.status().as_u16(), 200);

    let body: SearchResult = response.json().await.expect("Failed to parse response.");

    assert!(body.fish_result.len() >= 1);
    // This could be false unless app is created with a recipe pre-defined.
    // assert!(body.recipe_result.len() >= 1);

    assert!(body
        .fish_result
        .iter()
        .map(|fish| fish.id)
        .collect::<Vec<uuid::Uuid>>()
        .contains(&app.fish_type.id))
}
