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
