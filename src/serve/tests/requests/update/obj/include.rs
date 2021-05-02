use super::*;

lazy_static::lazy_static! {
    pub static ref UPDATE_BODY: serde_json::Value =
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "age": 100
            })
        })
    });
}

crate::run_test_request!(
    include_multi_rel_m2m,
    Method::PATCH,
    format!(
        "/public__peoples/{}?include=public__people-article",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    include_multi_rel_m2o,
    Method::PATCH,
    format!(
        "/public__peoples/{}?include=public__articles",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    include_single_rel,
    Method::PATCH,
    format!(
        "/public__peoples/{}?include=public__favorite_color",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    include_single_rel_sparsing_all,
    Method::PATCH,
    format!(
        "/public__peoples/{}?include=public__favorite_color&fields[public__favorite_color]=color",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    include_single_rel_sparsing_none,
    Method::PATCH,
    format!(
        "/public__peoples/{}?include=public__favorite_color&fields[public__favorite_color]=",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);
