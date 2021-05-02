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
    type_,
    Method::PATCH,
    "/public__peoples/",
    400,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    type_related,
    Method::PATCH,
    format!(
        "/public__peoples/{}/public__articles",
        PEOPLES_IDS[0].to_string()
    ),
    400,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    type_id,
    Method::PATCH,
    format!("/public__favorite_color/{}", PEOPLES_IDS[0].to_string()),
    200,
    UPDATE_BODY.clone()
);
