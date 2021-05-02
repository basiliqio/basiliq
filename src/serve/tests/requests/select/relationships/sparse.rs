use super::*;

crate::run_test_request!(
    sparse_self_single,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles?fields[public__peoples]=first-name",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sparse_self_multi,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles?fields[public__peoples]=first-name,last-name,age",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sparse_self_empty,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles?fields[public__peoples]=",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sparse_other_single,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles?fields[public__articles]=title",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sparse_other_multi,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles?fields[public__articles]=title,body",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sparse_other_empty,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles?include=public__peoples&fields[public__peoples]=",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sparse_other_all_empty,
    Method::GET,
    format!("/public__peoples/{}/relationships/public__articles?include=public__peoples&fields[public__articles]=&fields[public__peoples]=", PEOPLES_IDS[0]),
    200
);
