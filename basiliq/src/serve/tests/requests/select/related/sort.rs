use super::*;

crate::run_test_request!(
    sort_local_single,
    Method::GET,
    format!(
        "/public__peoples/{}/public__articles?sort=title",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sort_local_multi,
    Method::GET,
    format!(
        "/public__peoples/{}/public__articles?sort=-body,title",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sort_remote_single,
    Method::GET,
    format!(
        "/public__articles/{}/public__peoples?sort=public__favorite_color.color",
        ARTICLES_IDS[1]
    ),
    200
);

crate::run_test_request!(
    sort_remote_single_multi,
    Method::GET,
    format!(
        "/public__articles/{}/public__peoples?sort=-public__favorite_color.color,-first-name",
        ARTICLES_IDS[0]
    ),
    200
);
