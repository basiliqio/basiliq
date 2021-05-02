use super::*;

#[basiliq_test(run_migrations)]
async fn bad_content_type(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let mut request = prepare_basiliq_request(Method::GET, "/an_unknown_resource", Body::empty());
    request.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    handle_errors(resp, BasiliqErrorId::BadContentType).await;
}

#[basiliq_test(run_migrations)]
async fn bad_method(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(Method::PUT, "/public__peoples", Body::empty());

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    handle_errors(resp, BasiliqErrorId::BadMethod).await;
}

#[basiliq_test(run_migrations)]
async fn bad_json(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from("AAAAAAAAAAAAAAA"),
    );

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::BadJson).await;
}

#[basiliq_test(run_migrations)]
async fn bad_uuid(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(Method::GET, "/public__peoples/AAAAAAAA", Body::empty());

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    handle_errors(resp, BasiliqErrorId::BadUuid).await;
}
