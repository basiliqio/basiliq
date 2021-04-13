use super::*;
use crate::cli::config::generate::BasiliqCliGenerateConfig;
use basiliq_database_scanner::{BasiliqDbScannedTable, BasiliqStoreBuilder};
use log::{error, info};
use std::fs::OpenOptions;

pub async fn gen_config(
    param: &BasiliqCliResult,
    opt: &BasiliqCliGenerateConfig,
) -> Result<(), BasiliqError> {
    let mut connection =
        crate::database::connection::get_single_connection(param.database_connection_infos())
            .await?;
    let mut open_opt = OpenOptions::new();
    open_opt.write(true);
    match opt.overwrite() {
        true => open_opt.create(true),
        false => open_opt.create_new(true),
    };
    let file = match open_opt.open(opt.path().clone()) {
        Ok(x) => x,
        Err(err) if matches!(err.kind(), std::io::ErrorKind::AlreadyExists) => {
            error!(
                "File '{}' already exists. Use option '-w' to overwrite.",
                opt.path().to_string_lossy()
            );
            return Ok(());
        }
        Err(err) => return Err(BasiliqError::IO(err)),
    };

    info!("Scanning the database...");
    let builder = BasiliqStoreBuilder::new(BasiliqDbScannedTable::scan_db(&mut connection).await?);
    let config = builder.config();
    serde_yaml::to_writer(file, config)?;
    info!(
        "Configuration file written to {}",
        opt.path().to_string_lossy()
    );
    Ok(())
}