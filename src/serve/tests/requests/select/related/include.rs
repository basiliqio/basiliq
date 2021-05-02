use super::*;

crate::run_test_request!(
    include_rel_m2o,
    Method::GET,
    format!(
        "/public__articles/{}/public__peoples?include=public__favorite_color",
        ARTICLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    include_self,
    Method::GET,
    format!(
        "/public__peoples/{}/public__articles?include=public__peoples",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    include_rel_o2m,
    Method::GET,
    format!(
        "/public__peoples/{}/public__articles?include=public__people-article",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    include_distant,
    Method::GET,
    format!(
        "/public__peoples/{}/public__articles?include=public__peoples.public__comments",
        PEOPLES_IDS[0]
    ),
    200
);
crate::run_test_request!(
    include_distant_self,
    Method::GET,
    format!(
        "/public__peoples/{}/public__articles?include=public__peoples.public__comments.public__articles",
        PEOPLES_IDS[0]
    ),
    200
);
