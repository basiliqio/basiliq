pub mod creation;
mod errors;
mod messy_json_to_str;
mod request_type;
mod utils;
mod value;

use ciboulette::*;
pub use errors::Ciboulette2SqlError;
use messy_json::{MessyJson, MessyJsonObjectValue, MessyJsonValue};
use std::borrow::Cow;
use std::collections::BTreeMap;

// pub use request_type::query_type_selector;

#[derive(Clone, Debug)]
pub enum Ciboulette2SqlStep<'a> {
    Main,
    Relationships(&'a str),
}

#[derive(Clone, Debug, Default)]
pub struct Ciboulette2SqlArguments<'a>(Vec<quaint::ast::Value<'a>>);

impl<'a> Ciboulette2SqlArguments<'a> {
    pub fn new(params: Vec<quaint::ast::Value<'a>>) -> Self {
        Ciboulette2SqlArguments(params)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Ciboulette2SqlRequest<'a> {
    request: String,
    params: Ciboulette2SqlArguments<'a>,
}

//FIXME https://github.com/rust-lang/rust/issues/82219
#[derive(sqlx::FromRow, Debug)]
pub struct Ciboulette2SqlResultWithId {
    id: String,
    #[sqlx(rename = "v")]
    value: serde_json::value::Value,
}

//FIXME https://github.com/rust-lang/rust/issues/82219
#[derive(sqlx::FromRow, Debug)]
pub struct Ciboulette2SqlResult {
    #[sqlx(rename = "v")]
    value: serde_json::value::Value,
}

// impl<'a> Ciboulette2SqlSingleResult<'a>
// {
// 	pub fn value(&'a self) ->  &'a serde_json::value::RawValue
// 	{
// 		self.value
// 	}
// }

#[derive(sqlx::FromRow)]
pub struct Ciboulette2SqlMultiResult<'a> {
    values: Vec<&'a serde_json::value::RawValue>,
}

impl<'a> Ciboulette2SqlMultiResult<'a> {
    pub fn values(&'a self) -> &'a Vec<&'a serde_json::value::RawValue> {
        &self.values
    }
}

#[macro_export]
macro_rules! json_wrap {
    ($req:ident) => {
        format!("WITH q AS ({}) SELECT ROW_TO_JSON(q) AS v FROM q;", $req)
    };
}

#[macro_export]
macro_rules! json_wrap_with_id {
    ($req:ident) => {
        format!(
            "WITH q AS ({}) SELECT id::text, ROW_TO_JSON(q) AS v FROM q;",
            $req
        )
    };
}
