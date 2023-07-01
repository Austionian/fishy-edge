use crate::helpers::spawn_app;

#[tokio::test]
async fn fish_route_should_return_the_fish_by_id() {
    let app = spawn_app().await;

    let response = app.get_fish_by_id(app.fish.id).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn fish_route_should_return_400_for_invalid_uuid() {
    let app = spawn_app().await;

    let fake_id = uuid::Uuid::new_v4();

    let response = app.get_fish_by_id(fake_id).await;

    assert_eq!(response.status().as_u16(), 500);
}
