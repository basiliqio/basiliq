use super::postgres_metadata::parsed::*;
use super::postgres_metadata::raw::*;
use super::*;
use getset::Getters;
use messy_json::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
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
pub use config::*;

#[derive(Debug, Clone)]
pub struct BasiliqStore<'a> {
    pub(crate) ciboulette: ciboulette::CibouletteStore<'a>,
    pub(crate) tables: ciboulette2postgres::Ciboulette2PostgresTableStore<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreTableIdentifier {
    schema_name: String,
    table_name: String,
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

#[derive(Debug, Clone, PartialEq, Ord, Eq, PartialOrd, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipIdentifier {
    table_id: BasiliqStoreTableIdentifier,
    field_name: String,
    index: usize,
}

impl From<&BasiliqStoreRelationshipData> for BasiliqStoreRelationshipIdentifier {
    fn from(data: &BasiliqStoreRelationshipData) -> Self {
        BasiliqStoreRelationshipIdentifier {
            table_id: data.ftable_name().clone(),
            field_name: data.ffield_name().clone(),
            index: 0,
        }
    }
}

impl BasiliqStoreRelationshipIdentifier {
    pub fn check_index(
        &mut self,
        relationships: &BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
    ) {
        while relationships.contains_key(&self) {
            self.index += 1;
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
    ftable_name: BasiliqStoreTableIdentifier,
    ffield_name: String,
    type_: BasiliqStoreRelationshipType,
}
