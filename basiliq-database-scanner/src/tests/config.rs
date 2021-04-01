use super::*;

async fn apply_migrations(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE peoples(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies(
			id			UUID PRIMARY KEY,
			title		TEXT NOT NULL
		);
		"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies_staff(
			id			UUID PRIMARY KEY,
			role		TEXT NOT NULL,
			person		UUID NOT NULL REFERENCES peoples(id) ON DELETE CASCADE,
			movies		UUID NOT NULL REFERENCES movies(id) ON DELETE CASCADE
		);
	"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
}

#[ciboulette2postgres_test]
async fn config_default(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    apply_migrations(&mut transaction).await;
    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    let config = BasiliqStoreConfig::from(&builder);
    insta::assert_json_snapshot!(config);
}
