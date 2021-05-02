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
    sort_local_single,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?sort=color",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);
