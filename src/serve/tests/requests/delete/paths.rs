use super::*;

crate::run_test_request!(
    deleting_related,
    Method::DELETE,
    format!("/public__peoples/{}/public__articles", PEOPLES_IDS[0]),
    400
);
