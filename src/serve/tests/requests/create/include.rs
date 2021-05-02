use super::*;

lazy_static::lazy_static! {
    pub static ref CREATE_BODY: serde_json::Value =
    json!({
        "data": json!({
            "type": "public__peoples",
            "attributes": json!({
                "first-name": "AAAAA",
                "last-name": "BBBBBBB"
            })
        })
    });
}

async fn create_with_favorite_color(pool: sqlx::PgPool, query: &str) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__favorite_color",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__favorite_color",
                    "attributes": json!({
                        "color": "rot"
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
    let res = handle_response(resp).await;
    let color_id = res
        .as_object()
        .and_then(|x| x.get("data"))
        .and_then(serde_json::Value::as_object)
        .and_then(|x| x.get("id"))
        .unwrap()
        .clone();
    let request = prepare_basiliq_request(
        Method::POST,
        query,
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "attributes": json!({
                        "first-name": "AAAAA",
                        "last-name": "BBBBBBB"
                    }),
                    "relationships": json!({
                        "public__favorite_color": json!({
                            "data": json!({
                                "id": color_id,
                                "type": "public__favorite_color"
                            })
                        })
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
    let res = handle_response(resp).await;
    crate::test_json!(res);
}

crate::run_test_request!(
    include_multi_rel_m2m,
    Method::POST,
    "/public__peoples?include=public__people-article",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    include_multi_rel_m2o,
    Method::POST,
    "/public__peoples?include=public__articles",
    201,
    CREATE_BODY.clone()
);

#[basiliq_test(run_migrations)]
async fn include_single_rel(pool: sqlx::PgPool) {
    create_with_favorite_color(pool, "/public__peoples?include=public__favorite_color").await;
}

#[basiliq_test(run_migrations)]
async fn include_single_rel_sparsing_all(pool: sqlx::PgPool) {
    create_with_favorite_color(
        pool,
        "/public__peoples?include=public__favorite_color&fields[public__favorite_color]=color",
    )
    .await;
}

#[basiliq_test(run_migrations)]
async fn include_single_rel_sparsing_none(pool: sqlx::PgPool) {
    create_with_favorite_color(
        pool,
        "/public__peoples?include=public__favorite_color&fields[public__favorite_color]=",
    )
    .await;
}
