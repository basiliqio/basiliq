use super::*;

lazy_static::lazy_static! {
    pub static ref UPDATE_BODY: serde_json::Value =
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "age": 100
            })
        })
    });
}

crate::run_test_request!(
    no_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}?fields[public__peoples]=",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    one_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}?fields[public__peoples]=first-name",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    multi_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}?fields[public__peoples]=first-name,gender",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    unknown_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}?fields[public__peoples]=AAAAAAAAAAAAA",
        PEOPLES_IDS[0]
    ),
    400,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    unknown_type,
    Method::PATCH,
    format!(
        "/public__peoples/{}?fields[AAAAAAAAAAAAAA]=gender",
        PEOPLES_IDS[0]
    ),
    400,
    UPDATE_BODY.clone()
);
