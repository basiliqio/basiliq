use super::*;
use serde::{Deserialize, Serialize};

/// Top level of the Store configuration
///
/// Contains a list of accepted resources
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BasiliqStoreConfig {
    resources: BTreeMap<String, BasiliqStoreResourceConfig>,
}

/// The configuration of a store resource
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BasiliqStoreResourceConfig {
    /// The schema name of the resource
    schema: String,
    /// The table name of the resource
    table_name: String,
    enabled: bool,
    /// A map of the relationships
    relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig>,
}

/// The configuration of a store relationships
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BasiliqStoreRelationshipsConfig {
    target: String,
    field: String,
}
