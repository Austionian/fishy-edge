use crate::helpers::spawn_app;

#[tokio::test]
async fn you_shoud_be_able_to_see_your_favorite_fish_and_recipes() {
    let app = spawn_app().await;

    let response = app.get_favorites().await;

    assert_eq!(response.status().as_u16(), 200);
}
