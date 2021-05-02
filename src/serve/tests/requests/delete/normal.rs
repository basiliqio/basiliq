use super::*;

crate::run_test_request!(
    simple_delete,
    Method::DELETE,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    200
);

crate::run_test_request!(
    delete_simple_rel,
    Method::DELETE,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color",
        PEOPLES_IDS[0]
    ),
    200
);
