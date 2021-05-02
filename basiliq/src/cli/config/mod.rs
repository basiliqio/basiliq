use super::*;

pub mod check;
pub mod generate;

/// Handle the CLI for command relative to the configuration management
pub async fn handle_cli(
    connect_option: BasiliqDbConnectionOption,
    cli_matches: &ArgMatches<'_>,
) -> Option<BasiliqCliResult> {
    match cli_matches.subcommand() {
        ("generate", Some(x)) => generate::handle_cli(connect_option, x).await,
        ("check", Some(x)) => check::handle_cli(connect_option, x).await,
        _ => unreachable!(),
    }
}
