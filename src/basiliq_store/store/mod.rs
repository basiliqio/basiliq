use super::postgres_metadata::parsed::*;
use super::postgres_metadata::raw::*;
use super::*;
use bimap::BiBTreeMap;
use ciboulette::CibouletteIdType;
use getset::{CopyGetters, Getters, MutGetters};
use messy_json::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
use tracing::{trace, warn};

/// The system column we shouldn't expose
const POSTGRES_SYSTEM_COLUMNS: &[&str] =
    &["oid", "tableoid", "xmin", "cmin", "xmax", "cmax", "ctid"];

/// Postgres system schema NOT to include in the API
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

/// A store for the Basiliq project
///
/// It contains:
///
/// - a [CibouletteStore](ciboulette::CibouletteStore) necessary to parse incoming request and building response
/// - a [Ciboulette2PgTableStore](ciboulette2pg::Ciboulette2PgTableStore) necessary to execute `Postgres` queries
/// - a configuration ([BasiliqStoreConfig](BasiliqStoreConfig)) for the current store.
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStore {
    pub(crate) ciboulette: ciboulette::CibouletteStore,
    pub(crate) tables: ciboulette2pg::Ciboulette2PgTableStore,
    pub(crate) config: BasiliqStoreConfig,
}

/// A identifier for a table. Made of the table's schema and the table's name
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct BasiliqStoreTableIdentifier {
    /// Table schema
    schema: String,
    /// Table name
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

/// A list of possible relationships type, utils when building [BasiliqStore](BasiliqStore)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasiliqStoreRelationshipType {
    /// One-to-many relationship. Take a value, true if part of Many-to-Many relationships
    OneToMany(bool),
    /// Many-to-one relationship. Take a value, true if part of Many-to-Many relationships
    ManyToOne(bool),
    /// Many-to-Many relationship. Take an option structure
    ManyToMany(BasiliqStoreRelationshipManyToManyData),
}

/// Data about a Many-to-Many relationships beeing built.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipManyToManyData {
    /// The table identifier of the bucket table
    bucket: BasiliqStoreTableIdentifier,
    /// The field name of the current table
    lfield_name: ArcStr,
    /// The field of the other table
    ffield_name: ArcStr,
}

/// Generic data about a relationship
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters, CopyGetters)]
pub struct BasiliqStoreRelationshipData {
    /// The identifier of the current table
    #[getset(get = "pub")]
    ltable: BasiliqStoreTableIdentifier,
    /// The field name of the current table beeing mapped
    #[getset(get = "pub")]
    lfield_name: ArcStr,
    /// The distant table identifier
    #[getset(get = "pub")]
    ftable: BasiliqStoreTableIdentifier,
    /// The distant table field name
    #[getset(get = "pub")]
    ffield_name: ArcStr,
    /// The relationship type
    #[getset(get = "pub")]
    type_: BasiliqStoreRelationshipType,
    /// True if the relationship is optional
    #[getset(get_copy = "pub")]
    optional: bool,
}
