use super::*;
use serde_json::{json, Value};
mod create;
mod update;
use basiliq_db_test_utils::*;

#[macro_export]
macro_rules! test_json {
	($value:ident) => {
		insta::assert_json_snapshot!($value,
		{
			".**.id" => insta::dynamic_redaction(crate::serve::tests::check_uuid),
		});
	};
}

pub async fn handle_response<'a>(response: Response<Body>) -> serde_json::Value {
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let res: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    res
}
