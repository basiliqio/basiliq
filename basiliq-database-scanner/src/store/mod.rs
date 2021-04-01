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

#[derive(Debug, Clone)]
pub struct BasiliqStore<'a> {
    pub(crate) ciboulette: ciboulette::CibouletteStore<'a>,
    pub(crate) tables: ciboulette2postgres::Ciboulette2PostgresTableStore<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct BasiliqStoreTableIdentifier {
    schema_name: String,
    table_name: String,
}

impl BasiliqStoreTableIdentifier {
    pub fn new(schema_name: &str, table_name: &str) -> Self {
        BasiliqStoreTableIdentifier {
            schema_name: schema_name.to_string(),
            table_name: table_name.to_string(),
        }
    }
}

impl std::fmt::Display for BasiliqStoreTableIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.schema_name, self.table_name)
    }
}

impl From<&BasiliqDbScannedTable> for BasiliqStoreTableIdentifier {
    fn from(table: &BasiliqDbScannedTable) -> Self {
        BasiliqStoreTableIdentifier {
            table_name: table.table().name().clone(),
            schema_name: table.schema().name().clone(),
        }
    }
}

impl From<&BasiliqStoreResourceConfig> for BasiliqStoreTableIdentifier {
    fn from(table: &BasiliqStoreResourceConfig) -> Self {
        BasiliqStoreTableIdentifier {
            table_name: table.target().table_name().clone(),
            schema_name: table.target().schema_name().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasiliqStoreRelationshipType {
    OneToMany,
    ManyToOne,
    ManyToMany(BasiliqStoreTableIdentifier),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipData {
    ltable_name: BasiliqStoreTableIdentifier,
    lfield_name: String,
    ftable_name: BasiliqStoreTableIdentifier,
    ffield_name: String,
    type_: BasiliqStoreRelationshipType,
}
