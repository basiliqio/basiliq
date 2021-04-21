use super::*;

#[basiliq_test(run_migrations)]
async fn single_rel(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__favorite_color",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__favorite_color",
                    "attributes": json!({
                        "color": "rot"
                    })
                })
            })
            .to_string(),
        ),
    );

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let res = handle_create(resp).await;
    crate::test_json!(res);
    let color_id = res.as_object().and_then(|x| x.get("id")).unwrap().clone();
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "attributes": json!({
                        "first-name": "AAAAA",
                        "last-name": "BBBBBBB"
                    }),
                    "relationships": json!({
                        "public__favorite_color": json!({
                            "data": json!({
                                "id": color_id,
                                "type": "public__favorite_color"
                            })
                        })
                    })
                })
            })
            .to_string(),
        ),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let res = handle_create(resp).await;
    crate::test_json!(res);
}

#[basiliq_test(run_migrations)]
async fn multi_rel(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__articles",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__articles",
                    "attributes": json!({
                        "title": "Hallo!",
                        "body": "Holaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    })
                })
            })
            .to_string(),
        ),
    );

    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let res = handle_create(resp).await;
    crate::test_json!(res);
    let articles_id = res.as_object().and_then(|x| x.get("id")).unwrap().clone();
    let request = prepare_basiliq_request(
        Method::POST,
        "/public__peoples",
        Body::from(
            json!({
                "data": json!({
                    "type": "public__peoples",
                    "attributes": json!({
                        "first-name": "AAAAA",
                        "last-name": "BBBBBBB"
                    }),
                    "relationships": json!({
                        "public__articles": json!({
                            "data": json!({
                                "id": articles_id,
                                "type": "public__articles"
                            })
                        })
                    })
                })
            })
            .to_string(),
        ),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    handle_errors(
        resp,
        BasiliqErrorId::Ciboulette2PostgresManyRelationshipDirectWrite,
    )
    .await;
}
