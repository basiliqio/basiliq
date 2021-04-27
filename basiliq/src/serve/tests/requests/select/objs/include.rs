use super::*;

crate::run_test_request!(
    include_rel_m2o,
    Method::GET,
    "/public__peoples?include=public__favorite_color",
    200
);
crate::run_test_request!(
    include_rel_m2m,
    Method::GET,
    "/public__peoples?include=public__articles",
    200
);
crate::run_test_request!(
    include_rel_o2m,
    Method::GET,
    "/public__peoples?include=public__people-article",
    200
);

crate::run_test_request!(
    include_distant,
    Method::GET,
    "/public__peoples?include=public__articles.public__comments",
    200
);
crate::run_test_request!(
    include_distant_self,
    Method::GET,
    "/public__peoples?include=public__articles.public__comments.public__peoples",
    200
);
