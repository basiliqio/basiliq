use super::*;
use getset::CopyGetters;

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct BasiliqCliGenerateConfig {
    #[getset(get = "pub")]
    path: std::path::PathBuf,
    #[getset(get_copy = "pub")]
    overwrite: bool,
}

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
