use super::*;
use crate::cli::config::check::BasiliqCliCheckConfig;
use basiliq_database_scanner::{
    BasiliqDbScannedTable, BasiliqStoreBuilder, BasiliqStoreConfig, BasiliqStoreConfigMergeable,
};
use log::{error, info};
use std::fs::File;

pub async fn check_config(
    param: &BasiliqCliResult,
    opt: &BasiliqCliCheckConfig,
) -> Result<(), BasiliqError> {
    let file = match File::open(opt.path().clone()) {
        Ok(x) => x,
        Err(err) => {
            error!("Failed to read configuration: {}", err);
            return Ok(());
        }
    };
    let read_config: BasiliqStoreConfig = match serde_yaml::from_reader(file) {
        Ok(x) => x,
        Err(err) => {
            error!("Failed to deserialize configuration {}", err);
            return Ok(());
        }
    };
    if opt.scan() {
        let mut connection =
            crate::database::connection::get_single_connection(param.database_connection_infos())
                .await?;
        info!("Scanning the database...");
        let mut builder =
            BasiliqStoreBuilder::new(BasiliqDbScannedTable::scan_db(&mut connection).await?);
        match builder.basiliq_config_merge(&read_config) {
            Ok(_) => (),
            Err(err) => {
                error!("{}", err);
            }
        }
    }
    info!("Configuration is valid.");
    Ok(())
}
