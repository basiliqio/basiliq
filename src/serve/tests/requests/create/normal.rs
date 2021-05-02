use serde_json::Value;

use super::*;

async fn create_people(pool: sqlx::PgPool, val: Value, expected_status: StatusCode) -> Value {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(val.to_string()),
    );

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();

    assert_eq!(resp.status(), expected_status);
    handle_response(resp).await
}

#[basiliq_test(run_migrations)]
async fn only_mandatory_fields(pool: sqlx::PgPool) {
    let res = create_people(
        pool,
        json!({
            "data": json!({
                "type": "public__peoples",
                "attributes": json!({
                    "first-name": "Francis",
                    "last-name": "it's me",
                })
            })
        }),
        StatusCode::CREATED,
    )
    .await;
    crate::test_json!(res);
}

#[basiliq_test(run_migrations)]
async fn with_optional_fields(pool: sqlx::PgPool) {
    let res = create_people(
        pool,
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
        }),
        StatusCode::CREATED,
    )
    .await;
    crate::test_json!(res);
}

#[basiliq_test(run_migrations)]
async fn missing_field(pool: sqlx::PgPool) {
    let res = create_people(
        pool,
        json!({
            "data": json!({
                "type": "public__peoples",
                "attributes": json!({
                    "first-name": "Francis",
                })
            })
        }),
        StatusCode::BAD_REQUEST,
    )
    .await;
    crate::test_json!(res);
}

#[basiliq_test(run_migrations)]
async fn unknown_key(pool: sqlx::PgPool) {
    let res = create_people(
        pool,
        json!({
            "data": json!({
                "type": "public__peoples",
                "attributes": json!({
                    "first-name": "Francis",
                    "last-name": "it's me",
                    "AAAAAAA": "AIE"
                })
            })
        }),
        StatusCode::BAD_REQUEST,
    )
    .await;
    crate::test_json!(res);
}

#[basiliq_test(run_migrations)]
async fn bad_type(pool: sqlx::PgPool) {
    let res = create_people(
        pool,
        json!({
            "data": json!({
                "type": "public__favorite_color",
                "attributes": json!({
                    "color": "Francis",
                })
            })
        }),
        StatusCode::BAD_REQUEST,
    )
    .await;
    crate::test_json!(res);
}
