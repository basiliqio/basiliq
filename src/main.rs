use anyhow::{ensure, Result};
use sqlx::postgres::PgPoolOptions;
mod postgres_metadata;

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
    postgres_metadata::raw::read_schemas(&pool).await?;
    postgres_metadata::raw::read_tables(&pool).await?;
    postgres_metadata::raw::read_columns(&pool).await?;
    postgres_metadata::raw::read_types(&pool).await?;
    let basiliq_table = postgres_metadata::parsed::BasiliqTable::new(
        postgres_metadata::raw::read_schemas(&pool).await?,
        postgres_metadata::raw::read_tables(&pool).await?,
        postgres_metadata::raw::read_columns(&pool).await?,
        postgres_metadata::raw::read_types(&pool).await?,
        postgres_metadata::raw::read_primary_keys(&pool).await?,
        postgres_metadata::raw::read_foreign_keys(&pool).await?,
    )?;
    println!("Result {:#?}", basiliq_table);
    Ok(())
}
