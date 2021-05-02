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
    sorting_local,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color?sort=color",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

// crate::run_test_request!(
//     sorting_local_multi,
//     Method::PATCH,
//     format!(
//         "/public__peoples/{}/relationships/public__favorite_color?sort=first-name,-last-name",
//         PEOPLES_IDS[0]
//     ),
//     200,
//     UPDATE_BODY.clone()
// );

// crate::run_test_request!(
//     sorting_remote,
//     Method::PATCH,
//     format!(
//         "/public__peoples/{}/relationships/public__favorite_color?sort=-public__favorite_color.color",
//         PEOPLES_IDS[0]
//     ),
//     200,
//     UPDATE_BODY.clone()
// );

// crate::run_test_request!(
//     sorting_remote_multi_rel,
//     Method::PATCH,
//     format!(
//         "/public__peoples/{}/relationships/public__favorite_color?sort=public__articles.title",
//         PEOPLES_IDS[0]
//     ),
//     403,
//     UPDATE_BODY.clone()
// );

// crate::run_test_request!(
//     sorting_and_include_and_sparse,
//     Method::PATCH,
//     format!("/public__peoples/{}/relationships/public__favorite_color?sort=public__favorite_color.color&include=public__favorite_color&fields[public__favorite_color]=", PEOPLES_IDS[0]),
//     200,
//     UPDATE_BODY.clone()
// );
