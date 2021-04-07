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
    database_connection_infos: PgConnectOptions,
    intention: BasiliqCliIntention,
}

pub async fn handle_cli<'a>() -> Option<BasiliqCliResult> {
    let yaml = clap::load_yaml!("clap/base.yml");
    let clap_app = clap::App::from_yaml(yaml).version(VERSION);
    let cli_matches = clap_app.get_matches();
    let connect_option = database_connection::gen_database_url(&cli_matches);

    match cli_matches.subcommand() {
        ("config", Some(x)) => config::handle_cli(connect_option, x).await,
        ("serve", Some(x)) => serve::handle_cli(connect_option, x).await,
        _ => unreachable!(),
    }
}
