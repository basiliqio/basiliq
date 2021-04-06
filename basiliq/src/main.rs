mod cli;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let cli_res = cli::handle_cli().await;
    match cli_res {
        Some(cli_param) => {
            println!("CLI : {:#?}", cli_param);
        }
        None => (),
    };
    Ok(())
}
