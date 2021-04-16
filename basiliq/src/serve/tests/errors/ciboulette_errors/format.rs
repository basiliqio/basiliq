use super::*;

#[basiliq_test(run_migrations)]
async fn main_type_clash(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__articles", // This
                    "attributes": json!(
                    {
                        "body": "toto",
                        "title": "tutu"
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
    handle_errors(resp, BasiliqErrorId::CibouletteMainTypeClash).await;
}

#[basiliq_test(run_migrations)]
async fn unknown_type(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(Method::GET, "/AAAAAAAAAAA", Body::empty());
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteUnknownType).await;
}

#[basiliq_test(run_migrations)]
async fn unknown_relationships(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::GET,
        "/public__favorite_color?include=public__comments",
        Body::empty(),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteUnknownRelationship).await;
}

#[basiliq_test(run_migrations)]
async fn unknown_fields(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::GET,
        "/public__peoples?fields[public__peoples]=AAAAAA",
        Body::empty(),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteUnknownField).await;
}

#[basiliq_test(run_migrations)]
async fn nested_sorting(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::GET,
        "/public__peoples?sort=articles.comments.body",
        Body::empty(),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteNestedSorting).await;
}

#[basiliq_test(run_migrations)]
async fn no_data(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "meta": json!({
                    "something": "HAHAH",
                })
            })
            .to_string(),
        ),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteNoData).await;
}

#[basiliq_test(run_migrations)]
async fn missing_id(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::PATCH,
        "/public__peoples/f781f789-e701-4edc-a66c-8f7e19036b7c",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "attributes": json!({
                        "first-name": "Francis"
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
    handle_errors(resp, BasiliqErrorId::CibouletteMissingId).await;
}

#[basiliq_test(run_migrations)]
async fn bad_id_type(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!([{
                    "type": "public__peoples",
                    "id": "0db3c22c-d20d-4c23-9b01-0f0cc1e465f7",
                    "attributes": json!({
                        "first-name": "Francis",
                        "last-name": "it's me",
                    })
                },
                {
                    "type": "public__peoples",
                    "id": "fde25783-ff24-49e3-ba2b-00b5697222df",
                    "attributes": json!({
                        "first-name": "Francis",
                        "last-name": "it's not me",
                    })
                },
                ])
            })
            .to_string(),
        ),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteNoCompound).await;
}

#[basiliq_test(run_migrations)]
async fn no_compound(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!([{
                    "type": "public__peoples",
                    "id": "0db3c22c-d20d-4c23-9b01-0f0cc1e465f7",
                    "attributes": json!({
                        "first-name": "Francis",
                        "last-name": "it's me",
                    })
                },
                {
                    "type": "public__peoples",
                    "id": "fde25783-ff24-49e3-ba2b-00b5697222df",
                    "attributes": json!({
                        "first-name": "Francis",
                        "last-name": "it's not me",
                    })
                },
                ])
            })
            .to_string(),
        ),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteNoCompound).await;
}

#[basiliq_test(run_migrations)]
async fn missing_type_in_path(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/", // this
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "id": "0db3c22c-d20d-4c23-9b01-0f0cc1e465f7",
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
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteMissingTypeInPath).await;
}

#[basiliq_test(run_migrations)]
async fn bad_path(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::GET,
        "/public__peoples/0db3c22c-d20d-4c23-9b01-0f0cc1e465f7/cantevenspellrelationships/public__articles", // this
        Body::empty(),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteBadPath).await;
}

#[basiliq_test(run_migrations)]
async fn wrong_path_type(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples/0db3c22c-d20d-4c23-9b01-0f0cc1e465f7", // this
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "id": "0db3c22c-d20d-4c23-9b01-0f0cc1e465f7",
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
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::CibouletteWrongPathType).await;
}
