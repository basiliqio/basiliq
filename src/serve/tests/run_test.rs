use super::*;

pub async fn run_request(
    state: Arc<BasiliqServerState>,
    method: hyper::Method,
    query: String,
    expected_status: u16,
    body: Option<serde_json::Value>,
) -> serde_json::Value {
    let request = prepare_basiliq_request(
        method,
        query.as_str(),
        match body {
            Some(x) => Body::from(x.to_string()),
            None => Body::empty(),
        },
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        hyper::StatusCode::from_u16(expected_status).unwrap()
    );
    let bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let res: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    res
}

#[macro_export]
macro_rules! run_test_request {
    ($name:ident, $method:expr, $query:expr, $expected_status:literal) => {
        #[basiliq_test(run_migrations, init_values)]
        async fn $name(pool: sqlx::PgPool) {
            let state = prepare_basiliq_test(pool).await;

            let res = crate::serve::tests::run_request(state, $method, $query.into(), $expected_status, None).await;
			insta::assert_json_snapshot!(res);
        }
    };

	($name:ident, $method:expr, $query:expr, $expected_status:literal, $body:expr) => {
		#[basiliq_test(run_migrations, init_values)]
		async fn $name(pool: sqlx::PgPool) {
			let state = prepare_basiliq_test(pool).await;

			let res = crate::serve::tests::run_request(state, $method, $query.into(), $expected_status, Some($body)).await;
			insta::assert_json_snapshot!(res,
			{
				".**.id" => insta::dynamic_redaction(crate::serve::tests::check_uuid),
			});
		}
	};
}
