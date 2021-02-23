use anyhow::{ensure, Result};
use sqlx::postgres::PgPoolOptions;
mod openapi;
mod postgres_metadata;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::init();
	let db_uri = std::env::var_os("DATABASE_URL");
	ensure!(db_uri.is_some() == true, "No database url is set");
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(db_uri.unwrap().to_str().unwrap())
		.await?;
	println!("Connected");
	let (schemas, tables, columns, types, primary_keys, foreign_keys) = try_join!(
		postgres_metadata::raw::read_schemas(&pool),
		postgres_metadata::raw::read_tables(&pool),
		postgres_metadata::raw::read_columns(&pool),
		postgres_metadata::raw::read_types(&pool),
		postgres_metadata::raw::read_primary_keys(&pool),
		postgres_metadata::raw::read_foreign_keys(&pool)
	)?;
	let basiliq_table = postgres_metadata::parsed::BasiliqTable::new(
		schemas,
		tables,
		columns,
		types,
		primary_keys,
		foreign_keys,
	)?;
	let account = sqlx::query!("select (1) as id, 'Herp Derpinson' as name")
		.fetch_one(&pool)
		.await?;

	// anonymous struct has `#[derive(Debug)]` for convenience
	println!("{:?}", account);
	// println!("Result {:#?}", basiliq_table);
	Ok(())
}
