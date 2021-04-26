use super::*;
use basiliq_db_test_utils::*;

lazy_static::lazy_static! {
    pub static ref CREATE_BODY_PEOPLES: serde_json::Value =
        json!({
            "data": json!({
                "type": "public__peoples",
                "attributes": json!({
                    "first-name": "AAAAA",
                    "last-name": "BBBBBBB"
                })
            })
        });

        pub static ref CREATE_BODY_ARTICLES: serde_json::Value =
        json!({
            "data": json!({
                "type": "public__articles",
                "attributes": json!({
                    "title": "AAAAAAAAAAAAAAAAAA",
                    "body": "BBBBBBBBB"
                })
            })
        });
}

crate::run_test_request!(
    type_related,
    Method::POST,
    format!(
        "/public__peoples/{}/public__articles",
        PEOPLES_IDS[0].to_string()
    ),
    400,
    CREATE_BODY_ARTICLES.clone()
);

crate::run_test_request!(
    type_id,
    Method::POST,
    format!("/public__peoples/{}", PEOPLES_IDS[0].to_string()),
    400,
    CREATE_BODY_PEOPLES.clone()
);

crate::run_test_request!(
    type_relationships,
    Method::POST,
    format!(
        "/public__peoples/{}/relationships/public__articles",
        PEOPLES_IDS[0].to_string()
    ),
    400,
    CREATE_BODY_ARTICLES.clone()
);
