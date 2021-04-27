use super::*;

crate::run_test_request!(
    sort_local_single,
    Method::GET,
    format!("/public__peoples/{}?sort=first-name", PEOPLES_IDS[0]),
    200
);

crate::run_test_request!(
    sort_local_multi,
    Method::GET,
    format!("/public__peoples/{}?sort=-first-name", PEOPLES_IDS[0]),
    200
);

crate::run_test_request!(
    sort_remote_single,
    Method::GET,
    format!(
        "/public__peoples/{}?sort=public__favorite_color.color",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sort_remote_single_multi,
    Method::GET,
    format!(
        "/public__peoples/{}?sort=-public__favorite_color.color,-first-name",
        PEOPLES_IDS[0]
    ),
    200
);
