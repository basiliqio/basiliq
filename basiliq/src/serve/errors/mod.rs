mod ciboulette_errors;
mod enums;
mod ill_requests;
mod mappers;
pub mod obj;

use ciboulette::{CibouletteErrorObj, CibouletteErrorRequest};
use enums::BasiliqErrorId;
use hyper::{Body, Response};
use obj::BasiliqServerError;
use std::borrow::Cow;
