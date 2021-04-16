use super::*;

#[basiliq_test(run_migrations)]
async fn only_mandatory_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "attributes": json!({
                        "first-name": "Francis",
                        "last-name": "it's me",
                    })
                })
            })
            .to_string(),
        ),
    );

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let res = handle_create(resp).await;
    crate::test_json!(res);
}

#[basiliq_test(run_migrations)]
async fn with_optional_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "attributes": json!({
                        "first-name": "Francis",
                        "last-name": "it's me",
                        "twitter": "@myhandle",
                        "gender": "M",
                        "age": 22
                    })
                })
            })
            .to_string(),
        ),
    );

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let res = handle_create(resp).await;
    crate::test_json!(res);
}
