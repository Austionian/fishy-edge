use crate::helpers::spawn_app;

#[tokio::test]
async fn a_user_shoud_be_able_to_get_a_min_and_max_value_for_a_lake() {
    let app = spawn_app().await;

    let response = app.get_min_and_max("Michigan", "protein").await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn a_user_should_be_able_to_use_all_valid_lakes() {
    let app = spawn_app().await;

    const LAKES: [&str; 4] = ["Michigan", "Huron", "Superior", "Store"];

    for lake in LAKES {
        let response = app.get_min_and_max(lake, "protein").await;

        assert_eq!(response.status().as_u16(), 200);
    }
}

#[tokio::test]
async fn a_user_should_be_able_to_use_all_valid_attrs() {
    let app = spawn_app().await;

    const ATTRS: [&str; 4] = ["protein", "pcb", "mercury", "omega_3_ratio"];

    for attr in ATTRS {
        let response = app.get_min_and_max("Michigan", attr).await;

        assert_eq!(response.status().as_u16(), 200);
    }
}

#[tokio::test]
async fn an_invalid_lake_is_not_acceptable() {
    let app = spawn_app().await;

    let response = app.get_min_and_max("Lake", "protien").await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn an_invalid_attr_is_not_acceptable() {
    let app = spawn_app().await;

    let response = app.get_min_and_max("Michigan", "attr").await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn lake_is_an_optional_query_param() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/min_and_max?attr=protein", &app.address))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Unable to get min and max without lake param.");

    print!("{:?}", response);
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn lake_is_an_optional_query_param_but_attr_needs_to_be_valid() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/v1/min_and_max?attr=attr", &app.address))
        .header("Authorization", &format!("Bearer {}", &app.api_key))
        .send()
        .await
        .expect("Unable to get min and max without lake param.");

    assert_eq!(response.status().as_u16(), 400);
}
