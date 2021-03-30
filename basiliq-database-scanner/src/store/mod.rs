use super::postgres_metadata::parsed::*;
use super::postgres_metadata::raw::*;
use super::*;
use messy_json::*;
use std::borrow::Cow;
use std::collections::BTreeMap;

mod builder;
mod objects;

pub use builder::BasiliqStoreBuilder;

#[derive(Debug, Clone)]
pub struct BasiliqStore<'a> {
    pub(crate) ciboulette: ciboulette::CibouletteStore<'a>,
    pub(crate) tables: ciboulette2postgres::Ciboulette2PostgresTableStore<'a>,
}
