use super::*;
use crate::cli::config::check::BasiliqCliCheckConfig;
use basiliq_database_scanner::{
    BasiliqDbScannedTable, BasiliqStoreBuilder, BasiliqStoreConfig, BasiliqStoreConfigMergeable,
};
use log::{error, info};
use std::fs::File;
use std::path::PathBuf;

pub fn read_config_from_file(path: PathBuf) -> Result<BasiliqStoreConfig, BasiliqError> {
    let file = match File::open(path) {
        Ok(x) => x,
        Err(err) => {
            error!("Failed to read configuration: {}", err);
            return Err(BasiliqError::from(err));
        }
    };
    let read_config: BasiliqStoreConfig = match serde_yaml::from_reader(file) {
        Ok(x) => x,
        Err(err) => {
            error!("Failed to deserialize configuration {}", err);
            return Err(BasiliqError::from(err));
        }
    };
    Ok(read_config)
}

pub async fn create_store_builder_single_conn<'a>(
    mut conn: sqlx::PgConnection,
    read_config: BasiliqStoreConfig,
) -> Result<BasiliqStoreBuilder<'a>, BasiliqError> {
    info!("Scanning the database...");
    let mut builder = BasiliqStoreBuilder::new(BasiliqDbScannedTable::scan_db(&mut conn).await?);
    builder.basiliq_config_merge(&read_config)?;
    Ok(builder)
}

pub async fn create_store_builder_pool<'a>(
    pool: &sqlx::PgPool,
    config_path: PathBuf,
) -> Result<BasiliqStoreBuilder<'a>, BasiliqError> {
    let read_config = read_config_from_file(config_path)?;
    // let connection = conn.acquire().await?;
    info!("Scanning the database...");
    let mut builder =
        BasiliqStoreBuilder::new(BasiliqDbScannedTable::scan_db_pool(pool.clone()).await?);
    builder.basiliq_config_merge(&read_config)?;
    info!("Configuration is valid.");
    Ok(builder)
}

pub async fn check_config(
    param: &BasiliqCliResult,
    opt: &BasiliqCliCheckConfig,
) -> Result<(), BasiliqError> {
    let read_config = read_config_from_file(opt.path().clone())?;
    if opt.scan() {
        let connection =
            crate::database::connection::get_single_connection(param.database_connection_infos())
                .await?;
        create_store_builder_single_conn(connection, read_config).await?;
    }
    info!("Configuration is valid.");
    Ok(())
}
