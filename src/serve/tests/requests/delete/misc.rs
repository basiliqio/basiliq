use super::*;

crate::run_test_request!(
    sorting_local,
    Method::DELETE,
    format!("/public__peoples/{}?sort=first-name", PEOPLES_IDS[0]),
    200
);

crate::run_test_request!(
    sorting_local_multi,
    Method::DELETE,
    format!(
        "/public__peoples/{}?sort=first-name,-last-name",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sorting_remote,
    Method::DELETE,
    format!(
        "/public__peoples/{}?sort=-public__favorite_color.color",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sorting_remote_multi_rel,
    Method::DELETE,
    format!(
        "/public__peoples/{}?sort=public__articles.title",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    sorting_and_include_and_sparse,
    Method::DELETE,
    format!("/public__peoples/{}?sort=public__favorite_color.color&include=public__favorite_color&fields[public__favorite_color]=", PEOPLES_IDS[0]),
    200
);
