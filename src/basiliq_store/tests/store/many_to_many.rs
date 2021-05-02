use super::*;
use serde_json::json;

#[basiliq_test]
async fn many_to_many_relationships_check(mut pool: sqlx::PgPool) {
    let store = setup_n_m(&mut pool, None).await;
    let (_, rel) = store
        .ciboulette()
        .get_rel("public__movies", "public__peoples")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToMany(rel) => {
            assert_eq!(rel.bucket_resource().name(), "public__movie_staff");
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("public__movies").unwrap())
                    .unwrap(),
                "movie"
            );
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("public__peoples").unwrap())
                    .unwrap(),
                "staff"
            );
        }
        _ => panic!("Wrong rel type"),
    };

    let (_, rel) = store
        .ciboulette()
        .get_rel("public__movies", "public__movie_staff")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::OneToMany(rel) => {
            assert_eq!(rel.many_resource().name(), "public__movie_staff");
            assert_eq!(rel.many_resource_key(), "movie");
            assert_eq!(rel.one_resource().name(), "public__movies")
        }
        _ => panic!("Wrong rel type"),
    };

    let (_, rel) = store
        .ciboulette()
        .get_rel("public__peoples", "public__movies")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToMany(rel) => {
            assert_eq!(rel.bucket_resource().name(), "public__movie_staff");
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("public__movies").unwrap())
                    .unwrap(),
                "movie"
            );
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("public__peoples").unwrap())
                    .unwrap(),
                "staff"
            );
        }
        _ => panic!("Wrong rel type"),
    };

    let (_, rel) = store
        .ciboulette()
        .get_rel("public__peoples", "public__movie_staff")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::OneToMany(rel) => {
            assert_eq!(rel.many_resource().name(), "public__movie_staff");
            assert_eq!(rel.many_resource_key(), "staff");
            assert_eq!(rel.one_resource().name(), "public__peoples")
        }
        _ => panic!("Wrong rel type"),
    };
    assert_eq!(
        store
            .ciboulette()
            .get_type("public__movies")
            .unwrap()
            .relationships()
            .len(),
        2
    );
    assert_eq!(
        store
            .ciboulette()
            .get_type("public__peoples")
            .unwrap()
            .relationships()
            .len(),
        2
    );
}

#[basiliq_test]
async fn many_to_many_relationships_check_with_config(mut transaction: sqlx::PgPool) {
    let store = setup_n_m(
        &mut transaction,
        Some(
            serde_json::from_value(json!({
                "resources": json!({
                    "peoples": json!({
                        "target": json!({
                            "table": "peoples",
                            "schema": "public"
                        }),
                        "enabled": true,
                        "relationships": json!({
                            "movies": json!({
                                "target": json!({
                                    "table": "movies",
                                    "schema": "public"
                                }),
                                "through": json!({
                                    "table": "movie_staff",
                                    "schema": "public",
                                    "field": "staff"
                                }),
                                "field": "id",
                                "enabled": true,
                            }),
                            "movie_staff": json!({
                                "target": json!({
                                    "table": "movie_staff",
                                    "schema": "public"
                                }),
                                "field": "staff",
                                "enabled": true,
                            })
                        })
                    }),
                    "movies": json!({
                        "target": json!({
                            "table": "movies",
                            "schema": "public"
                        }),
                        "enabled": true,
                        "relationships": json!({
                            "staff": json!({
                                "target": json!({
                                    "table": "peoples",
                                    "schema": "public"
                                }),
                                "through": json!({
                                    "table": "movie_staff",
                                    "schema": "public",
                                    "field": "movie"
                                }),
                                "field": "id",
                                "enabled": true,
                            }),
                            "movie_staff": json!({
                                "target": json!({
                                    "table": "movie_staff",
                                    "schema": "public"
                                }),
                                "field": "movie",
                                "enabled": true,
                            })
                        })
                    }),
                    "movie_staff": json!({
                        "target": json!({
                            "table": "movie_staff",
                            "schema": "public"
                        }),
                        "enabled": true,
                        "relationships": json!({
                            "staff": json!({
                                "target": json!({
                                    "table": "peoples",
                                    "schema": "public"
                                }),
                                "field": "staff",
                                "enabled": true,
                            }),
                            "movie": json!({
                                "target": json!({
                                    "table": "movies",
                                    "schema": "public"
                                }),
                                "field": "movie",
                                "enabled": true,
                            })
                        })
                    })
                })
            }))
            .unwrap(),
        ),
    )
    .await;
    let (_, rel) = store.ciboulette().get_rel("movies", "staff").unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToMany(rel) => {
            assert_eq!(rel.bucket_resource().name(), "movie_staff");
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("movies").unwrap())
                    .unwrap(),
                "movie"
            );
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("peoples").unwrap())
                    .unwrap(),
                "staff"
            );
        }
        _ => panic!("Wrong rel type"),
    };

    let (_, rel) = store.ciboulette().get_rel("movies", "movie_staff").unwrap();
    match rel {
        CibouletteRelationshipOption::OneToMany(rel) => {
            assert_eq!(rel.many_resource().name(), "movie_staff");
            assert_eq!(rel.many_resource_key(), "movie");
            assert_eq!(rel.one_resource().name(), "movies")
        }
        _ => panic!("Wrong rel type"),
    };

    let (_, rel) = store.ciboulette().get_rel("peoples", "movies").unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToMany(rel) => {
            assert_eq!(rel.bucket_resource().name(), "movie_staff");
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("movies").unwrap())
                    .unwrap(),
                "movie"
            );
            assert_eq!(
                rel.keys_for_type(store.ciboulette().get_type("peoples").unwrap())
                    .unwrap(),
                "staff"
            );
        }
        _ => panic!("Wrong rel type"),
    };

    let (_, rel) = store
        .ciboulette()
        .get_rel("peoples", "movie_staff")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::OneToMany(rel) => {
            assert_eq!(rel.many_resource().name(), "movie_staff");
            assert_eq!(rel.many_resource_key(), "staff");
            assert_eq!(rel.one_resource().name(), "peoples")
        }
        _ => panic!("Wrong rel type"),
    };
    assert_eq!(
        store
            .ciboulette()
            .get_type("movies")
            .unwrap()
            .relationships()
            .len(),
        2
    );
    assert_eq!(
        store
            .ciboulette()
            .get_type("peoples")
            .unwrap()
            .relationships()
            .len(),
        2
    );
}
