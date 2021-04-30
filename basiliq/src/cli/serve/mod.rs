use super::*;
use getset::CopyGetters;
use std::path::PathBuf;
use tracing::error;

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct BasiliqCliServerConfig {
    #[getset(get = "pub")]
    pub(crate) bind_address: String,
    #[getset(get_copy = "pub")]
    pub(crate) bind_port: u16,
    #[getset(get = "pub")]
    pub(crate) config_file: Option<PathBuf>,
    #[getset(get_copy = "pub")]
    pub(crate) demo_mode: bool,
}

pub async fn handle_cli(
    connect_option: BasiliqDbConnectionOption,
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
            demo_mode: cli_matches.is_present("demo"),
            config_file: match cli_matches.value_of("config") {
                Some(config) => Some(
                    PathBuf::from_str(config)
                        .expect("The configuration file should've been a valid path"),
                ),
                None => match cli_matches.is_present("dynamic_config") {
                    true => None,
                    false => {
                        error!("The configuration file should be specified or the --dynamic-config option");
                        std::process::exit(1);
                    }
                },
            },
        }),
    })
}
