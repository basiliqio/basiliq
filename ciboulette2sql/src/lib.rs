mod creation;
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

#[derive(Clone, Debug)]
pub enum Ciboulette2SqlStep<'a> {
    Main,
    Relationships(&'a str),
}

#[derive(Clone, Debug, Default)]
pub struct Ciboulette2SqlRequest<'a> {
    requests: Vec<Vec<(String, Vec<quaint::ast::Value<'a>>)>>,
}

impl<'a> Ciboulette2SqlRequest<'a> {
    pub fn with_capacity(size: usize) -> Self {
        Ciboulette2SqlRequest {
            requests: Vec::with_capacity(size),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.requests.append(&mut other.requests);
    }
}
