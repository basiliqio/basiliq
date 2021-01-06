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
	println!("Schemas {:#?}", postgres_metadata::read_schemas(&pool).await?);
	println!("Tables {:#?}", postgres_metadata::read_tables(&pool).await?);
	println!("Roles {:#?}", postgres_metadata::read_roles(&pool).await?);
	println!("Columns {:#?}", postgres_metadata::read_columns(&pool).await?);
	println!("Types {:#?}", postgres_metadata::read_types(&pool).await?);
    Ok(())
}
