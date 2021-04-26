use super::*;

crate::run_test_request!(
    obj_single_field,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    200,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "first-name": "OH NO",
            })
        })
    })
);

crate::run_test_request!(
    obj_multiple_fields,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    200,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "first-name": "OH NO",
                "last-name": "OH YES",
                "age": 18,
                "gender": "O"
            })
        })
    })
);

crate::run_test_request!(
    obj_set_to_null,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    200,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "age": Value::Null
            })
        })
    })
);

crate::run_test_request!(
    set_mandatory_to_null,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    400,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "first-name": Value::Null
            })
        })
    })
);

crate::run_test_request!(
    add_relationship_single,
    Method::PATCH,
    format!("/public__peoples/{}", PEOPLES_IDS[0]),
    200,
    json!({
        "data": json!({
            "type": "public__peoples",
            "id": PEOPLES_IDS[0],
            "attributes": json!({
                "age": 100
            }),
            "relationships": json!({
                "public__favorite_color": json!({
                    "data": json!({
                        "id": FAVORITE_COLOR_IDS[2],
                        "type": "public__favorite_color"
                    })
                })
            })
        })
    })
);
