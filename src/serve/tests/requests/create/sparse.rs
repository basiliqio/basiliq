use super::*;

lazy_static::lazy_static! {
    pub static ref CREATE_BODY: serde_json::Value =
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
    });
}

crate::run_test_request!(
    no_fields,
    Method::POST,
    "/public__peoples?fields[public__peoples]=",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    one_fields,
    Method::POST,
    "/public__peoples?fields[public__peoples]=first-name",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    multi_fields,
    Method::POST,
    "/public__peoples?fields[public__peoples]=first-name,gender",
    201,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    unknown_fields,
    Method::POST,
    "/public__peoples?fields[public__peoples]=AAAAAAAAAAAAA",
    400,
    CREATE_BODY.clone()
);

crate::run_test_request!(
    unknown_type,
    Method::POST,
    "/public__peoples?fields[AAAAAAAAAAAAAA]=gender",
    400,
    CREATE_BODY.clone()
);
