use super::*;
mod convert;
mod errors;
mod mergeable;
pub use errors::BasiliqStoreConfigError;
pub use mergeable::BasiliqStoreConfigMergeable;

/// Top level of the Store configuration
///
/// Contains a list of accepted resources
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreConfig {
    pub(crate) resources: BTreeMap<String, BasiliqStoreResourceConfig>,
}

/// The configuration of a store resource
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreResourceConfig {
    pub(crate) target: BasiliqStoreRelationshipTargetConfig,
    pub(crate) enabled: bool,
    /// A map of the relationships
    pub(crate) relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig>,
}

/// The configuration of a store relationships
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipsConfig {
    pub(crate) target: BasiliqStoreRelationshipTargetConfig,
    pub(crate) enabled: bool,
    pub(crate) field: String,
}

/// The configuration of a store relationship target
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipTargetConfig {
    /// The schema name of the resource
    pub(crate) schema: String,
    /// The table name of the resource
    pub(crate) table_name: String,
}

impl std::fmt::Display for BasiliqStoreRelationshipTargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.schema, self.table_name)
    }
}

impl BasiliqStoreConfig {
    fn check_uniq(&self) -> Result<(), BasiliqStoreConfigError> {
        let mut name_set: BTreeSet<&BasiliqStoreRelationshipTargetConfig> = BTreeSet::new();

        for resource in self.resources.values() {
            if !name_set.insert(resource.target()) {
                return Err(BasiliqStoreConfigError::DuplicateTable(
                    resource.target().clone(),
                ));
            }
        }
        Ok(())
    }
    pub fn check(&self) -> Result<(), BasiliqStoreConfigError> {
        self.check_uniq()
    }
}
