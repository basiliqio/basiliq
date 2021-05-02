use super::*;

crate::run_test_request!(
    sparse_self_single,
    Method::GET,
    "/public__peoples?fields[public__peoples]=first-name",
    200
);

crate::run_test_request!(
    sparse_self_multi,
    Method::GET,
    "/public__peoples?fields[public__peoples]=first-name,last-name,age",
    200
);

crate::run_test_request!(
    sparse_self_empty,
    Method::GET,
    "/public__peoples?fields[public__peoples]=",
    200
);

crate::run_test_request!(
    sparse_other_single,
    Method::GET,
    "/public__peoples?include=public__articles&fields[public__articles]=title",
    200
);

crate::run_test_request!(
    sparse_other_multi,
    Method::GET,
    "/public__peoples?include=public__articles&fields[public__articles]=title,body",
    200
);

crate::run_test_request!(
    sparse_other_empty,
    Method::GET,
    "/public__peoples?include=public__articles&fields[public__articles]=",
    200
);

crate::run_test_request!(
    sparse_other_all_empty,
    Method::GET,
    "/public__peoples?include=public__articles&fields[public__articles]=&fields[public__peoples]=",
    200
);
