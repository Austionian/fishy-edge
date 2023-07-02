use crate::helpers::spawn_app;

#[tokio::test]
async fn you_can_retrieve_averages_for_a_fish_type() {
    let app = spawn_app().await;

    let response = app.get_fish_type_avg(&app.fish_type.id).await;

    println!("{:?}", response);
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn you_must_give_a_valid_uuid() {
    let app = spawn_app().await;

    let response = app.get_fish_type_avg(&uuid::Uuid::new_v4()).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn you_must_give_a_uuid() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/fish_avg?", &app.address))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to post recipe delete.");

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn you_must_give_a_user_id() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!(
            "{}/v1/fish_avg?fishtype_id={}",
            &app.address, &app.fish_type.id
        ))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Failed to post recipe delete.");

    assert_eq!(response.status().as_u16(), 500);
}
