use super::*;

crate::run_test_request!(
    get_multi,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__articles",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    get_single,
    Method::GET,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color",
        PEOPLES_IDS[0]
    ),
    200
);

crate::run_test_request!(
    get_unknown,
    Method::GET,
    format!(
        "/public__favorite_color/{}/relationships/public__articles",
        FAVORITE_COLOR_IDS[0]
    ),
    400
);
