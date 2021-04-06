mod cli;
mod config;
mod database;
mod errors;

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
            _ => unimplemented!(),
        },
        None => Ok(()),
    }
}
