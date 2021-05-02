use super::*;
use serde_json::json;

#[basiliq_test]
async fn many_to_one_relationships_check(mut transaction: sqlx::PgPool) {
    let store = setup_1_n(&mut transaction, None).await;
    let (_, rel) = store
        .ciboulette()
        .get_rel("public__movies", "public__director")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToOne(rel) => {
            assert_eq!(rel.many_resource().name(), "public__movies");
            assert_eq!(rel.many_resource_key(), "director");
            assert_eq!(rel.one_resource().name(), "public__director");
        }
        _ => panic!("Wrong rel type"),
    };
    let (_, rel) = store
        .ciboulette()
        .get_rel("public__movies", "public__director")
        .unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToOne(rel) => {
            assert_eq!(rel.many_resource().name(), "public__movies");
            assert_eq!(rel.many_resource_key(), "director");
            assert_eq!(rel.one_resource().name(), "public__director");
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
        1
    );
    assert_eq!(
        store
            .ciboulette()
            .get_type("public__director")
            .unwrap()
            .relationships()
            .len(),
        1
    );
}

#[basiliq_test]
async fn one_to_many_with_config(mut transaction: sqlx::PgPool) {
    let store = setup_1_n(
        &mut transaction,
        Some(
            serde_json::from_value(json!({
                "resources": json!({
                    "director": json!({
                        "target": json!({
                            "table": "director",
                            "schema": "public"
                        }),
                        "enabled": true,
                        "relationships": json!({
                            "movies": json!({
                                "target": json!({
                                    "table": "movies",
                                    "schema": "public"
                                }),
                                "field": "director",
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
                            "director": json!({
                                "target": json!({
                                    "table": "director",
                                    "schema": "public"
                                }),
                                "field": "director",
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
    let (_, rel) = store.ciboulette().get_rel("director", "movies").unwrap();
    match rel {
        CibouletteRelationshipOption::OneToMany(rel) => {
            assert_eq!(rel.many_resource().name(), "movies");
            assert_eq!(rel.many_resource_key(), "director");
            assert_eq!(rel.one_resource().name(), "director");
        }
        _ => panic!("Wrong rel type"),
    };
    let (_, rel) = store.ciboulette().get_rel("movies", "director").unwrap();
    match rel {
        CibouletteRelationshipOption::ManyToOne(rel) => {
            assert_eq!(rel.many_resource().name(), "movies");
            assert_eq!(rel.many_resource_key(), "director");
            assert_eq!(rel.one_resource().name(), "director");
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
        1
    );
    assert_eq!(
        store
            .ciboulette()
            .get_type("director")
            .unwrap()
            .relationships()
            .len(),
        1
    );
}
