use super::*;
use getset::CopyGetters;

/// The `generate` CLI subcommand configuration
#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct BasiliqCliGenerateConfig {
    /// The path of the configuration
    #[getset(get = "pub")]
    path: std::path::PathBuf,
    /// `true` to overwrite if there is an existing file
    #[getset(get_copy = "pub")]
    overwrite: bool,
}

/// Handle the CLI for the `generate` subcommand
pub async fn handle_cli(
    connect_option: BasiliqDbConnectionOption,
    cli_matches: &ArgMatches<'_>,
) -> Option<BasiliqCliResult> {
    let out_file = match cli_matches.value_of("output") {
        Some(file) => {
            std::path::PathBuf::from_str(file).expect("the destination path should've been correct")
        }
        None => std::env::current_dir()
            .expect("to access the current directory")
            .join("basiliq_config.yaml"),
    };
    Some(BasiliqCliResult {
        database_connection_infos: connect_option,
        intention: BasiliqCliIntention::GenConfig(BasiliqCliGenerateConfig {
            path: out_file,
            overwrite: cli_matches.is_present("overwrite"),
        }),
    })
}
