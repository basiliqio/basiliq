use super::*;

#[basiliq_test]
async fn simple_table_serial_id(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			SERIAL PRIMARY KEY,
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
    assert_eq!(table.properties().has_field("first_name"), true);
    assert_eq!(table.properties().has_field("last_name"), true);
    assert_eq!(table.properties().has_field("id"), false);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Number), true);
}

#[basiliq_test]
async fn simple_table_big_serial_id(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			BIGSERIAL PRIMARY KEY,
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
    assert_eq!(table.properties().has_field("first_name"), true);
    assert_eq!(table.properties().has_field("last_name"), true);
    assert_eq!(table.properties().has_field("id"), false);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Number), true);
}

#[basiliq_test]
async fn simple_table_uuid_id(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			UUID PRIMARY KEY,
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
    assert_eq!(table.properties().has_field("first_name"), true);
    assert_eq!(table.properties().has_field("last_name"), true);
    assert_eq!(table.properties().has_field("id"), false);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Uuid), true);
}

#[basiliq_test]
async fn simple_table_text_id(pool: sqlx::PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			TEXT PRIMARY KEY,
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
    assert_eq!(table.properties().has_field("first_name"), true);
    assert_eq!(table.properties().has_field("last_name"), true);
    assert_eq!(table.properties().has_field("id"), false);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Text), true);
}

#[basiliq_test]
async fn simple_table_varchar_id(pool: sqlx::PgPool) {
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
    assert_eq!(table.properties().has_field("first_name"), true);
    assert_eq!(table.properties().has_field("last_name"), true);
    assert_eq!(table.properties().has_field("id"), false);
    assert_eq!(table.relationships().len(), 0);
    assert_eq!(matches!(table.id_type(), CibouletteIdType::Text), true);
}
