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
    include_self,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?include=public__peoples",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    include_single_rel_sparsing_all,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?include=public__peoples&fields[public__peoples]=first-name",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    include_single_rel_sparsing_none,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?include=public__peoples&fields[public__peoples]=",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);
