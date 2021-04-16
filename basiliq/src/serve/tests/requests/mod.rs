use super::*;
use serde_json::json;
mod create;
fn check_uuid<'store, 'b>(
    value: insta::internals::Content,
    _path: insta::internals::ContentPath<'store>,
) -> &'b str {
    assert_eq!(
        value
            .as_str()
            .unwrap()
            .chars()
            .filter(|&c| c == '-')
            .count(),
        4
    );
    "[uuid]"
}

#[macro_export]
macro_rules! test_json {
	($value:ident) => {
		insta::assert_json_snapshot!($value,
		{
			".id" => insta::dynamic_redaction(check_uuid),
			".relationships.*.data.id" => insta::dynamic_redaction(check_uuid),
		});
	};
}
