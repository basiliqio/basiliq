use super::*;

#[basiliq_test(run_migrations)]
async fn no_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples?fields[public__peoples]=",
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

#[basiliq_test(run_migrations)]
async fn one_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples?fields[public__peoples]=first-name",
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

#[basiliq_test(run_migrations)]
async fn multi_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples?fields[public__peoples]=first-name,gender",
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

#[basiliq_test(run_migrations)]
async fn unknown_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples?fields[public__peoples]=AAAAAAAAAAAAA",
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

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteUnknownField).await;
}

#[basiliq_test(run_migrations)]
async fn unknown_type(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples?fields[AAAAAAAAAAAAAA]=gender",
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

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteUnknownType).await;
}
