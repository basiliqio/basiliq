use super::*;

crate::run_test_request!(
    obj_single_field,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color",
        PEOPLES_IDS[0]
    ),
    200,
    json!({
        "data": json!({
            "type": "public__favorite_color",
            "id": FAVORITE_COLOR_IDS[1]
        })
    })
);

crate::run_test_request!(
    obj_force_null,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__favorite_color",
        PEOPLES_IDS[0]
    ),
    200,
    json!({ "data": Value::Null })
);

crate::run_test_request!(
    obj_multi_rel_m2m,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__articles",
        PEOPLES_IDS[0]
    ),
    403,
    json!({
        "data": json!({
            "type": "public__articles",
            "id": ARTICLES_IDS[1]
        })
    })
);

crate::run_test_request!(
    obj_multi_rel_o2m,
    Method::PATCH,
    format!(
        "/public__peoples/{}/relationships/public__people-article",
        PEOPLES_IDS[0]
    ),
    403,
    json!({
        "data": json!({
            "type": "public__people-article",
            "id": PEOPLE_ARTICLE_ID[1]
        })
    })
);
