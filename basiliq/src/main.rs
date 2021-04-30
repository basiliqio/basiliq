mod cli;
mod config;
mod database;
mod errors;
mod logging;
mod serve;

use cli::{BasiliqCliIntention, BasiliqCliResult};
use errors::BasiliqError;

#[tokio::main]
pub async fn main() -> Result<(), BasiliqError> {
    logging::init_logging();

    let cli_res = cli::handle_cli().await;
    match cli_res {
        Some(cli_param) => match cli_param.intention() {
            BasiliqCliIntention::GenConfig(path) => {
                config::generate::gen_config(&cli_param, path).await
            }
            BasiliqCliIntention::CheckConfig(path) => {
                config::check::check_config(&cli_param, path).await
            }
            BasiliqCliIntention::Serve(opt) => serve::serve(&cli_param, opt.clone()).await,
        },
        None => Ok(()),
    }
}
