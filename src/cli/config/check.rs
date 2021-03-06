use super::*;
use getset::CopyGetters;

/// Configuration concerning the `check` subcommand
#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct BasiliqCliCheckConfig {
    /// The path of the configuration file to check
    #[getset(get = "pub")]
    path: std::path::PathBuf,
    /// `true` if should scan the database or just check the configuration file on its own
    #[getset(get_copy = "pub")]
    scan: bool,
}

/// Handle the CLI for the `check` subcommand
pub async fn handle_cli(
    connect_option: BasiliqDbConnectionOption,
    cli_matches: &ArgMatches<'_>,
) -> Option<BasiliqCliResult> {
    let out_file = match cli_matches.value_of("input") {
        Some(file) => {
            std::path::PathBuf::from_str(file).expect("the source path should've been correct")
        }
        None => std::env::current_dir()
            .expect("to access the current directory")
            .join("basiliq_config.yaml"),
    };
    Some(BasiliqCliResult {
        database_connection_infos: connect_option,
        intention: BasiliqCliIntention::CheckConfig(BasiliqCliCheckConfig {
            path: out_file,
            scan: !cli_matches.is_present("no_scan"),
        }),
    })
}
