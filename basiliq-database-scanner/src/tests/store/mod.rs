use super::*;

mod many_to_many;
mod one_to_many;
async fn setup_1_n(pool: &mut sqlx::PgPool, config: Option<BasiliqStoreConfig>) -> BasiliqStore {
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE director(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut *conn)
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
    .execute(&mut *conn)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *conn).await.unwrap();
    let mut builder = BasiliqStoreBuilder::new(raw_table);
    if let Some(config) = config {
        builder.basiliq_config_merge(&config).unwrap();
    }
    builder.build().unwrap()
}

async fn setup_n_m(pool: &mut sqlx::PgPool, config: Option<BasiliqStoreConfig>) -> BasiliqStore {
    let mut conn = pool.acquire().await.unwrap();

    sqlx::query!(
        r#"
		CREATE TABLE peoples(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut *conn)
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
    .execute(&mut *conn)
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
    .execute(&mut *conn)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut *conn).await.unwrap();
    let mut builder = BasiliqStoreBuilder::new(raw_table);
    if let Some(config) = config {
        builder.basiliq_config_merge(&config).unwrap();
    }
    builder.build().unwrap()
}
