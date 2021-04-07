mod cli;
mod config;
mod database;
mod errors;
mod serve;

use errors::BasiliqError;

use cli::{BasiliqCliIntention, BasiliqCliResult};

#[tokio::main]
pub async fn main() -> Result<(), BasiliqError> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("basiliq=info,warn"),
    )
    .init();
    let cli_res = cli::handle_cli().await;
    match cli_res {
        Some(cli_param) => match cli_param.intention() {
            BasiliqCliIntention::GenConfig(path) => {
                config::generate::gen_config(&cli_param, path).await
            }
            BasiliqCliIntention::CheckConfig(path) => {
                config::check::check_config(&cli_param, path).await
            }
            BasiliqCliIntention::Serve(opt) => serve::serve(&cli_param, opt).await,
        },
        None => Ok(()),
    }
}
