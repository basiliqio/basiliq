use super::*;

#[ciboulette2postgres_test]
async fn simple_table_serial_id(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			SERIAL PRIMARY KEY,
			first_name	TEXT,
			last_name	TEXT
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
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

#[ciboulette2postgres_test]
async fn simple_table_big_serial_id(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			BIGSERIAL PRIMARY KEY,
			first_name	TEXT,
			last_name	TEXT
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
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

#[ciboulette2postgres_test]
async fn simple_table_uuid_id(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			UUID PRIMARY KEY,
			first_name	TEXT,
			last_name	TEXT
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
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

#[ciboulette2postgres_test]
async fn simple_table_text_id(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			TEXT PRIMARY KEY,
			first_name	TEXT,
			last_name	TEXT
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
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

#[ciboulette2postgres_test]
async fn simple_table_varchar_id(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE simple_table(
			id			VARCHAR(10) PRIMARY KEY,
			first_name	TEXT,
			last_name	TEXT
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
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
