use crate::helpers::spawn_app;

#[tokio::test]
async fn the_mobile_client_should_be_able_to_get_everything() {
    let app = spawn_app().await;

    let response = app.get_everything().await;

    assert_eq!(response.status().as_u16(), 200);
}
