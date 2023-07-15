use crate::helpers::spawn_app;

#[tokio::test]
async fn a_user_should_be_able_get_an_url_to_upload_to_s3() {
    let app = spawn_app().await;

    let body = serde_json::json!({
        "name": "test_file"
    });

    let response = app.post_presign_url(body).await;

    assert_eq!(response.status().as_u16(), 200);
}
