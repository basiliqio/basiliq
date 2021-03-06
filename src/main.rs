//! # Introduction
//!
//! **_Basiliq_** is an REST API that expose a **_Postgres_** database without
//! requiring additional coding and/or configuration. The exposed API try to respect the
//! [`JSON:API`](https://jsonapi.org/format/) specification
//!
//! See the [repository](https://github.com/basiliqio/basiliq) for usage.
//!
//! ## Project structure
//!
//! The project is separated in crates and modules.
//!
//! The crates are :
//! - [Ciboulette](ciboulette): That handle parsing `JSON:API` requests and building responses
//! - [Ciboulette2Pg](ciboulette2pg): That handle querying the database on [Ciboulette](ciboulette) requests and building
//! [Ciboulette](ciboulette) responses
//! - [MessyJson](basiliq_store::messy_json): Deserializing dynamically declared `JSON`
//! - [BasiliqStore](basiliq_store): Scan the database to extract and parse its topology, allowing to build the configuration
//!
//! The modules of the current crate are:
//! - [cli](cli): Handle the command-line interface
//! - [config](config): Handle generating, checking and merging the configuration generated by [BasiliqStore](basiliq_store)
//! - [database](database): Handle database connection whether singular or pooled
//! - [logging](logging): Handle initializing the logging interface
//! - [serve](server): Handle the server management

#![warn(clippy::all)]

#[macro_use]
mod cli;
mod basiliq_store;
mod config;
mod database;
mod errors;
mod logging;
mod serve;

use cli::{BasiliqCliIntention, BasiliqCliResult};
use errors::BasiliqError;

#[tokio::main]
pub async fn main() -> Result<(), BasiliqError> {
    // Init the logging interface
    logging::init_logging();

    // Parse the command line interface
    let cli_res = cli::handle_cli().await;
    match cli_res {
        Some(cli_param) => match cli_param.intention() {
            BasiliqCliIntention::GenConfig(path) => {
                config::generate::gen_config(&cli_param, path).await
            }
            BasiliqCliIntention::CheckConfig(path) => {
                config::check::check_config(&cli_param, path).await
            }
            BasiliqCliIntention::Serve(opt) => serve::serve(&cli_param, opt.clone()).await,
        },
        None => Ok(()),
    }
}
