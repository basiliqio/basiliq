use super::*;

crate::run_test_request!(
    no_fields,
    Method::DELETE,
    format!(
        "/public__peoples/{}?fields[public__peoples]=",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    one_fields,
    Method::DELETE,
    format!(
        "/public__peoples/{}?fields[public__peoples]=first-name",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    multi_fields,
    Method::DELETE,
    format!(
        "/public__peoples/{}?fields[public__peoples]=first-name,gender",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    unknown_fields,
    Method::DELETE,
    format!(
        "/public__peoples/{}?fields[public__peoples]=AAAAAAAAAAAAA",
        PEOPLES_IDS[0]
    ),
    400
);

crate::run_test_request!(
    unknown_type,
    Method::DELETE,
    format!(
        "/public__peoples/{}?fields[AAAAAAAAAAAAAA]=gender",
        PEOPLES_IDS[0]
    ),
    400
);
