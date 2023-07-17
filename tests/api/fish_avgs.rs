use crate::helpers::spawn_app;

#[tokio::test]
async fn a_user_should_be_able_get_all_fish_type_avgs() {
    let app = spawn_app().await;

    let response = app.get_fish_type_avgs().await;

    assert_eq!(response.status().as_u16(), 200);
}
