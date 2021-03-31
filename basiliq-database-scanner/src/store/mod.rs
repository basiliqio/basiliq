use super::postgres_metadata::parsed::*;
use super::postgres_metadata::raw::*;
use super::*;
use messy_json::*;
use std::borrow::Cow;
use std::collections::BTreeMap;

const POSTGRES_SYSTEM_COLUMNS: &[&str] =
    &["oid", "tableoid", "xmin", "cmin", "xmax", "cmax", "ctid"];

const POSTGRES_SYSTEM_SCHEMA: &[&str] = &["pg_catalog", "pg_toast", "information_schema"];

mod builder;
mod keys;
mod name;
mod objects;

pub use builder::BasiliqStoreBuilder;
use builder::BasiliqStoreTableBuilder;

#[derive(Debug, Clone)]
pub struct BasiliqStore<'a> {
    pub(crate) ciboulette: ciboulette::CibouletteStore<'a>,
    pub(crate) tables: ciboulette2postgres::Ciboulette2PostgresTableStore<'a>,
}
