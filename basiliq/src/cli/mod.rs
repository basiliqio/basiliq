const VERSION: &str = env!("CARGO_PKG_VERSION");
use clap::ArgMatches;
use getset::Getters;
use sqlx::postgres::PgConnectOptions;
use std::str::FromStr;

pub mod config;
pub mod database_connection;
pub mod serve;

#[macro_export]
macro_rules! print_usage {
    ($cli:ident) => {
        println!("{}", $cli.usage());
        None
    };
}

#[derive(Clone, Debug)]
pub enum BasiliqCliIntention {
    GenConfig(config::generate::BasiliqCliGenerateConfig),
    CheckConfig(config::check::BasiliqCliCheckConfig),
    Serve(serve::BasiliqCliServerConfig),
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqCliResult {
    database_connection_infos: BasiliqDbConnectionOption,
    intention: BasiliqCliIntention,
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqDbConnectionOption {
    connection_option: PgConnectOptions,
    pool_max_connections: Option<usize>,
}

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
