use basiliq_database_scanner::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let db_uri = std::env::var_os("DATABASE_URL");
    anyhow::ensure!(db_uri.is_some() == true, "No database url is set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri.unwrap().to_str().unwrap())
        .await?;
    println!("Connected");
    let builder = BasiliqStoreBuilder::scan_db(pool).await?;
    println!("{:#?}", builder.build_object());
    Ok(())
}
