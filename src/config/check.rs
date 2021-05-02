use super::*;
use crate::basiliq_store::{
    BasiliqDbScannedTable, BasiliqStoreBuilder, BasiliqStoreConfig, BasiliqStoreConfigMergeable,
};
use crate::cli::config::check::BasiliqCliCheckConfig;
use std::fs::File;
use std::path::PathBuf;
use tracing::{error, info};

/// Read and deserialize a configuration providing a file path
pub fn read_config_from_file(path: PathBuf) -> Result<BasiliqStoreConfig, BasiliqError> {
    let file = match File::open(path.clone()) {
        Ok(x) => x,
        Err(err) => {
            error!("Failed to read configuration: {:#?}: {}", path, err);
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

/// Create a [BasiliqStoreBuilder](BasiliqStoreBuilder) from a single connection to the database
///
/// A configuration must be provided in that case
pub async fn create_store_builder_single_conn(
    mut conn: sqlx::PgConnection,
    read_config: BasiliqStoreConfig,
) -> Result<BasiliqStoreBuilder, BasiliqError> {
    info!("Scanning the database...");
    let mut builder = BasiliqStoreBuilder::new(BasiliqDbScannedTable::scan_db(&mut conn).await?);
    builder.basiliq_config_merge(&read_config)?;
    Ok(builder)
}

/// Create a [BasiliqStoreBuilder](BasiliqStoreBuilder) from a pool of connections to the database
///
/// A configuration might be provided, in which case the configuration provided will be merged with the scanned
/// configuration
pub async fn create_store_builder_pool(
    pool: &sqlx::PgPool,
    config_path: Option<PathBuf>,
) -> Result<BasiliqStoreBuilder, BasiliqError> {
    // let connection = conn.acquire().await?;
    info!("Scanning the database...");
    let mut builder =
        BasiliqStoreBuilder::new(BasiliqDbScannedTable::scan_db_pool(pool.clone()).await?);
    if let Some(config_path) = config_path {
        let read_config = read_config_from_file(config_path)?;
        builder.basiliq_config_merge(&read_config)?;
    }
    info!("Configuration is valid.");
    Ok(builder)
}

/// Check the configuration on it's own or against the database
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
