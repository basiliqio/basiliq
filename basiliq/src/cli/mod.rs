const VERSION: &str = env!("CARGO_PKG_VERSION");
use clap::ArgMatches;
use getset::Getters;
use sqlx::postgres::PgConnectOptions;
use std::str::FromStr;

pub mod config;
pub mod database_connection;
pub mod serve;

/// Type of CLI requests
#[derive(Clone, Debug)]
pub enum BasiliqCliIntention {
    /// Generate configuration
    GenConfig(config::generate::BasiliqCliGenerateConfig),
    /// Check an existing configuration, on its own or against an existing database
    CheckConfig(config::check::BasiliqCliCheckConfig),
    /// Start the server
    Serve(serve::BasiliqCliServerConfig),
}

/// The result of CLI parsing
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqCliResult {
    /// Structure holding informations to connect to the database and to maintain one or multiple connection to it
    database_connection_infos: BasiliqDbConnectionOption,
    /// The type of command that was expected by the CLI arguments
    intention: BasiliqCliIntention,
}

/// A collections of options for the database
///
/// To connect to the database and to maintain one or multiple database connection
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqDbConnectionOption {
    /// The connection options
    connection_option: PgConnectOptions,
    /// The max number of connection to have in the database pool
    pool_max_connections: Option<usize>,
}

/// Parse the CLI and return the result if any.
///
/// If `None` is returned, the parent function should return
pub async fn handle_cli<'a>() -> Option<BasiliqCliResult> {
    let yaml = clap::load_yaml!("clap/base.yml");
    let clap_app = clap::App::from_yaml(yaml).version(VERSION);
    let cli_matches = clap_app.get_matches();
    let connect_option = database_connection::gen_db_connection_options(&cli_matches);
    // let database_connection_option = database_connection::
    match cli_matches.subcommand() {
        ("config", Some(x)) => config::handle_cli(connect_option, x).await,
        ("serve", Some(x)) => serve::handle_cli(connect_option, x).await,
        _ => unreachable!(),
    }
}
