use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BasiliqStoreConfig {
    types: BTreeMap<String, BasiliqStoreResourceConfig>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BasiliqStoreResourceConfig {
    schema: String,
    table_name: String,
    enabled: bool,
    relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BasiliqStoreRelationshipsConfig {
    target: String,
    field: String,
}
