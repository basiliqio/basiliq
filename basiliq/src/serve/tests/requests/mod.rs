use super::*;
use serde_json::json;
mod create;

#[macro_export]
macro_rules! test_json {
	($value:ident) => {
		insta::assert_json_snapshot!($value,
		{
			".**.id" => insta::dynamic_redaction(crate::serve::tests::check_uuid),
		});
	};
}
