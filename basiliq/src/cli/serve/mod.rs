use super::*;
use getset::CopyGetters;
use std::path::PathBuf;

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct BasiliqCliServerConfig {
    #[getset(get = "pub")]
    bind_address: String,
    #[getset(get_copy = "pub")]
    bind_port: u16,
    #[getset(get = "pub")]
    config_file: PathBuf,
}

pub async fn handle_cli(
    connect_option: PgConnectOptions,
    cli_matches: &ArgMatches<'_>,
) -> Option<BasiliqCliResult> {
    Some(BasiliqCliResult {
        database_connection_infos: connect_option,
        intention: BasiliqCliIntention::Serve(BasiliqCliServerConfig {
            bind_address: cli_matches
                .value_of("bind_host")
                .unwrap_or("127.0.0.1")
                .to_string(),
            bind_port: cli_matches
                .value_of("bind_port")
                .map(|x| u16::from_str(x))
                .transpose()
                .expect("The port should've been a valid u16 number")
                .unwrap_or(8080),
            config_file: PathBuf::from_str(cli_matches.value_of("config").unwrap())
                .expect("The configuration file should've been a valid path"),
        }),
    })
}
