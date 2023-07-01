use crate::helpers::spawn_app;
use fishy_edge::routes::FishData;

#[tokio::test]
async fn fish_route_should_return_the_fish_by_id() {
    let app = spawn_app().await;

    let response = app.get_fish_by_id(app.fish.id).await;

    assert_eq!(response.status().as_u16(), 200);

    let response_body = response.json::<FishData>().await.unwrap();

    assert_eq!(&response_body.fish_data.name, &app.fish_type.name);
    assert_eq!(&response_body.fish_data.fish_type_id, &app.fish_type.id);
    assert_eq!(&response_body.fish_data.lake, &app.fish.lake);
    assert_eq!(&response_body.recipe_data.len(), &0);
    assert!(!&response_body.is_favorite);
}

#[tokio::test]
async fn fish_route_should_return_400_for_invalid_uuid() {
    let app = spawn_app().await;

    let fake_id = uuid::Uuid::new_v4();

    let response = app.get_fish_by_id(fake_id).await;

    assert_eq!(response.status().as_u16(), 400);
}
