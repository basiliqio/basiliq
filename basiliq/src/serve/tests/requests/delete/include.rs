use super::*;

crate::run_test_request!(
    include_multi_rel_m2o,
    Method::DELETE,
    format!(
        "/public__peoples/{}?include=public__favorite_color",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    include_multi_rel_m2m,
    Method::DELETE,
    format!(
        "/public__peoples/{}?include=public__people-article",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    include_multi_rel_o2m,
    Method::DELETE,
    format!(
        "/public__peoples/{}?include=public__articles",
        PEOPLES_IDS[0]
    ),
    200
);
