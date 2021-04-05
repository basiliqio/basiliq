use super::*;

mod many_to_many;
mod one_to_many;
async fn setup_1_n<'a>(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    config: Option<BasiliqStoreConfig>,
) -> BasiliqStore<'a> {
    sqlx::query!(
        r#"
		CREATE TABLE director(
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
			title		TEXT NOT NULL,
			director	UUID NOT NULL REFERENCES director(id) ON DELETE CASCADE
		);
	"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *transaction)
        .await
        .unwrap();
    let mut builder = BasiliqStoreBuilder::new(raw_table);
    if let Some(config) = config {
        builder.basiliq_config_merge(&config).unwrap();
    }
    builder.build().unwrap()
}

async fn setup_n_m<'a>(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    config: Option<BasiliqStoreConfig>,
) -> BasiliqStore<'a> {
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
		CREATE TABLE movie_staff(
			id			UUID PRIMARY KEY,
			staff		UUID NOT NULL REFERENCES peoples(id),
			movie		UUID NOT NULL REFERENCES movies(id)
		);
	"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *transaction)
        .await
        .unwrap();
    let mut builder = BasiliqStoreBuilder::new(raw_table);
    if let Some(config) = config {
        builder.basiliq_config_merge(&config).unwrap();
    }
    builder.build().unwrap()
}
