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

crate::run_test_request!(
    sorting_local,
    Method::POST,
    "/public__peoples?sort=first-name",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    sorting_local_multi,
    Method::POST,
    "/public__peoples?sort=first-name,-last-name",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    sorting_remote,
    Method::POST,
    "/public__peoples?sort=-public__favorite_color.color",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    sorting_remote_multi_rel,
    Method::POST,
    "/public__peoples?sort=public__articles.title",
    403,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    sorting_and_include_and_sparse,
    Method::POST,
    "/public__peoples?sort=public__favorite_color.color&include=public__favorite_color&fields[public__favorite_color]=",
    201,
    CREATE_BODY.clone()
);
