use super::*;

crate::run_test_request!(
    sort_local_single,
    Method::GET,
    "/public__peoples?sort=first-name",
    200
);

crate::run_test_request!(
    sort_local_multi,
    Method::GET,
    "/public__peoples?sort=-first-name",
    200
);

crate::run_test_request!(
    sort_remote_single,
    Method::GET,
    "/public__peoples?sort=public__favorite_color.color",
    200
);

crate::run_test_request!(
    sort_remote_single_multi,
    Method::GET,
    "/public__peoples?sort=-public__favorite_color.color,-first-name",
    200
);
