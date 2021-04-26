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
    sorting_local,
    Method::PATCH,
    format!("/public__peoples/{}?sort=first-name", PEOPLES_IDS[0]),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    sorting_local_multi,
    Method::PATCH,
    format!(
        "/public__peoples/{}?sort=first-name,-last-name",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    sorting_remote,
    Method::PATCH,
    format!(
        "/public__peoples/{}?sort=-public__favorite_color.color",
        PEOPLES_IDS[0]
    ),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    sorting_remote_multi_rel,
    Method::PATCH,
    format!(
        "/public__peoples/{}?sort=public__articles.title",
        PEOPLES_IDS[0]
    ),
    403,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    sorting_and_include_and_sparse,
    Method::PATCH,
    format!("/public__peoples/{}?sort=public__favorite_color.color&include=public__favorite_color&fields[public__favorite_color]=", PEOPLES_IDS[0]),
    200,
    UPDATE_BODY.clone()
);

crate::run_test_request!(
    add_relationship_m2m,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    403,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "age": 100
            }),
            "relationships": json!({
                "public__articles": json!({
                    "data": json!({
                        "id": ARTICLES_IDS[2],
                        "type": "public__articles"
                    })
                })
            })
        })
    })
);

crate::run_test_request!(
    add_relationship_m2o,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    403,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "age": 100
            }),
            "relationships": json!({
                "public__people-article": json!({
                    "data": json!({
                        "id": PEOPLE_ARTICLE_ID[2],
                        "type": "public__people-article"
                    })
                })
            })
        })
    })
);
