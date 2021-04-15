use super::*;
use std::ops::Deref;

#[basiliq_test]
async fn empty_db(pool: sqlx::PgPool) {
    let raw_table = BasiliqDbScannedTable::scan_db(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 0);
}

#[basiliq_test]
async fn simple_table_with_default_name(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			VARCHAR(10) PRIMARY KEY,
			first_name	TEXT,
			last_name	TEXT
		);
	"#
    )
    .execute(&mut *conn)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *conn).await.unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 1);
    let table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "simple_table"))
        .unwrap();
    let table_by_alias = builder.get_table_by_alias("public__simple_table").unwrap();
    assert_eq!(table == table_by_alias, true);
    assert_eq!(
        matches!(table.properties().properties().get("first_name").unwrap().deref(), messy_json::MessyJsonInner::String(x) if x.optional()),
        true
    );
    assert_eq!(
        matches!(table.properties().properties().get("last_name").unwrap().deref(), messy_json::MessyJsonInner::String(x) if x.optional()),
        true
    );
    assert_eq!(table.properties().has_field("id"), false);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Text), true);
}

#[basiliq_test]
async fn simple_table_with_no_field(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			VARCHAR(10) PRIMARY KEY
		);
	"#
    )
    .execute(&mut *conn)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *conn).await.unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 1);
    let table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "simple_table"))
        .unwrap();
    let table_by_alias = builder.get_table_by_alias("public__simple_table").unwrap();
    assert_eq!(table == table_by_alias, true);
    assert_eq!(table.properties().properties().is_empty(), true);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Text), true);
}
