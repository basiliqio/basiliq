mod ciboulette2pg_errors;
mod ciboulette_errors;
mod database_errors;
mod enums;
mod ill_requests;
mod mappers;

pub mod obj;
mod server_errors;

use ciboulette::{CibouletteErrorObj, CibouletteErrorRequest};
pub use enums::BasiliqErrorId;
use hyper::{Body, Response};
use obj::BasiliqServerError;
use std::borrow::Cow;

pub use mappers::convert_error_to_body;
