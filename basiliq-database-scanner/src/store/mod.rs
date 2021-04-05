use super::postgres_metadata::parsed::*;
use super::postgres_metadata::raw::*;
use super::*;
use bimap::BiBTreeMap;
use ciboulette::CibouletteIdType;
use getset::{Getters, MutGetters};
use log::{trace, warn};
use messy_json::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
const POSTGRES_SYSTEM_COLUMNS: &[&str] =
    &["oid", "tableoid", "xmin", "cmin", "xmax", "cmax", "ctid"];

const POSTGRES_SYSTEM_SCHEMA: &[&str] = &["pg_catalog", "pg_toast", "information_schema"];

mod builder;
mod config;
mod keys;
mod name;
mod objects;
mod relationships;

pub use builder::BasiliqStoreBuilder;
use builder::BasiliqStoreTableBuilder;
pub use config::{
    BasiliqStoreConfig, BasiliqStoreConfigError, BasiliqStoreConfigMergeable,
    BasiliqStoreRelationshipsConfig, BasiliqStoreResourceConfig,
};

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStore<'a> {
    pub(crate) ciboulette: ciboulette::CibouletteStore<'a>,
    pub(crate) tables: ciboulette2postgres::Ciboulette2PostgresTableStore<'a>,
    pub(crate) config: BasiliqStoreConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct BasiliqStoreTableIdentifier {
    schema: String,
    table: String,
}

impl BasiliqStoreTableIdentifier {
    pub fn new(schema_name: &str, table_name: &str) -> Self {
        BasiliqStoreTableIdentifier {
            schema: schema_name.to_string(),
            table: table_name.to_string(),
        }
    }
}

impl std::fmt::Display for BasiliqStoreTableIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.schema, self.table)
    }
}

impl From<&BasiliqDbScannedTable> for BasiliqStoreTableIdentifier {
    fn from(table: &BasiliqDbScannedTable) -> Self {
        BasiliqStoreTableIdentifier {
            table: table.table().name().clone(),
            schema: table.schema().name().clone(),
        }
    }
}

impl From<&BasiliqStoreResourceConfig> for BasiliqStoreTableIdentifier {
    fn from(table: &BasiliqStoreResourceConfig) -> Self {
        BasiliqStoreTableIdentifier {
            table: table.target().table().clone(),
            schema: table.target().schema().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasiliqStoreRelationshipType {
    /// One-to-many relationship. Take a value, true if part of Many-to-Many relationships
    OneToMany(bool),
    /// Many-to-one relationship. Take a value, true if part of Many-to-Many relationships
    ManyToOne(bool),
    /// Many-to-Many relationship. Take an option structure
    ManyToMany(BasiliqStoreRelationshipManyToManyData),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipManyToManyData {
    bucket: BasiliqStoreTableIdentifier,
    lfield_name: String,
    ffield_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipData {
    ltable: BasiliqStoreTableIdentifier,
    lfield_name: String,
    ftable: BasiliqStoreTableIdentifier,
    ffield_name: String,
    type_: BasiliqStoreRelationshipType,
}
