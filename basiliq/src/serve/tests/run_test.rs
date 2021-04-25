use super::*;

pub async fn run_request(
    state: Arc<BasiliqServerState>,
    method: hyper::Method,
    query: &str,
    expected_status: hyper::StatusCode,
    body: Option<serde_json::Value>,
) -> serde_json::Value {
    let request = prepare_basiliq_request(
        method,
        query,
        match body {
            Some(x) => Body::from(x.to_string()),
            None => Body::empty(),
        },
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), expected_status);
    let bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let res: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    crate::test_json!(res);
    res
}

#[macro_export]
macro_rules! run_test_request {
    ($name:literal, $method:literal, $query:literal, $expected_status:literal) => {
        #[basiliq_test(run_migrations)]
        async fn $name(pool: sqlx::PgPool) {
            let state = prepare_basiliq_test(pool).await;

            run_request(state, $method, $query, $expected_status, None).await
        }
    };

    ($name:literal, $method:literal, $query:literal, $expected_status:literal, $body:expr) => {
        #[basiliq_test(run_migrations)]
        async fn $name(pool: sqlx::PgPool) {
            let state = prepare_basiliq_test(pool).await;

            run_request(state, $method, $query, $expected_status, Some($body)).await
        }
    };
}
