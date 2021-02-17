mod create;
mod errors;
mod messy_json_to_str;
mod request_type;
mod utils;

use ciboulette::*;
use std::collections::BTreeMap;

pub use errors::Ciboulette2SqlError;
use messy_json::{MessyJson, MessyJsonObjectValue, MessyJsonValue};
use std::borrow::Cow;

pub use request_type::query_type_selector;

#[derive(Clone, Debug, Default)]
pub struct Ciboulette2SqlRequest<'a> {
    requests: Vec<(String, Vec<quaint::ast::Value<'a>>)>,
}
