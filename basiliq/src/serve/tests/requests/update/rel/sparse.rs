use super::*;

lazy_static::lazy_static! {
    pub static ref UPDATE_BODY: serde_json::Value =
    json!({
        "data": json!({
            "type": "public__favorite_color",
            "id": FAVORITE_COLOR_IDS[1]
        })
    });
}

crate::run_test_request!(
    no_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?fields[public__favorite_color]=",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    one_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?fields[public__favorite_color]=color",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    unknown_fields,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?fields[public__favorite_color]=AAAAAAAAAAAAA",
        PEOPLES_IDS[0]
    ),
    400,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    unknown_type,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?fields[AAAAAAAAAAAAAA]=gender",
        PEOPLES_IDS[0]
    ),
    400,
    UPDATE_BODY.clone()
);
