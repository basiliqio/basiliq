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
    let raw_tables = BasiliqDbScannedTable::scan_db_pool(pool).await?;
    let builder = BasiliqStoreBuilder::new(raw_tables);
    // println!("{:#?}", builder);
    // println!("{:#?}", BasiliqStoreConfig::from(&builder));
    let res = builder.build().unwrap();
    println!("{:#?}", res);
    Ok(())
}
