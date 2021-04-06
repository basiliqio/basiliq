use hyper::client::conn;

use super::*;

pub async fn handle_cli(
    connect_option: PgConnectOptions,
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
        intention: BasiliqCliIntention::GenConfig(out_file),
    })
}
