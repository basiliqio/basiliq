use super::*;

crate::run_test_request!(
    get,
    Method::GET,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    200
);
crate::run_test_request!(
    get_unknown,
    Method::GET,
    format!(
        "/public__peoples/{}",
        "3d046782-2c28-4b68-b148-f4b5afcd0984"
    ),
    404
);
