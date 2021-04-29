use super::*;

#[basiliq_test]
async fn self_rel_once(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE person(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL,
			mom			UUID REFERENCES person(id) ON DELETE CASCADE	
		);
	"#
    )
    .execute(&mut *conn)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *conn).await.unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 1);
    let person_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "person"))
        .unwrap();
    let person_table_self_rel = person_table.relationships().get("public__person").unwrap();

    assert_eq!(person_table_self_rel.ftable().table(), "person");
    assert_eq!(person_table_self_rel.ftable().schema(), "public");
    assert_eq!(person_table_self_rel.lfield_name(), "mom");
    assert_eq!(person_table_self_rel.ffield_name(), "id");
    assert_eq!(
        matches!(
            person_table_self_rel.type_(),
            BasiliqStoreRelationshipType::ManyToOne(_)
        ),
        true
    );
}

#[basiliq_test]
async fn self_rel_multi(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    let valid_keys = &["dad", "mom"];
    sqlx::query!(
        r#"
		CREATE TABLE person(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL,
			mom			UUID REFERENCES person(id) ON DELETE CASCADE,
			dad			UUID REFERENCES person(id) ON DELETE CASCADE	
		);
	"#
    )
    .execute(&mut *conn)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *conn).await.unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 1);
    let person_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "person"))
        .unwrap();
    let person_table_self_rel_mom = person_table.relationships().get("public__person").unwrap();

    assert_eq!(person_table_self_rel_mom.ftable().table(), "person");
    assert_eq!(person_table_self_rel_mom.ftable().schema(), "public");
    assert_eq!(
        valid_keys.contains(&person_table_self_rel_mom.lfield_name().as_str()),
        true
    );
    assert_eq!(person_table_self_rel_mom.ffield_name(), "id");
    assert_eq!(
        matches!(
            person_table_self_rel_mom.type_(),
            BasiliqStoreRelationshipType::ManyToOne(_)
        ),
        true
    );

    let person_table_self_dad = person_table
        .relationships()
        .get("public__person_0")
        .unwrap();

    assert_eq!(person_table_self_dad.ftable().table(), "person");
    assert_eq!(person_table_self_dad.ftable().schema(), "public");
    assert_eq!(
        valid_keys.contains(&person_table_self_dad.lfield_name().as_str()),
        true
    );
    assert_eq!(person_table_self_dad.ffield_name(), "id");
    assert_eq!(
        matches!(
            person_table_self_dad.type_(),
            BasiliqStoreRelationshipType::ManyToOne(_)
        ),
        true
    );
}
