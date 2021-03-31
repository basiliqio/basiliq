use super::*;

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

impl<'a> From<&BasiliqStoreBuilder<'a>> for BasiliqStoreConfig {
    /// Create a config object from a store builder
    fn from(builder: &BasiliqStoreBuilder<'a>) -> Self {
        let mut resources: BTreeMap<String, BasiliqStoreResourceConfig> = BTreeMap::new();

        for (alias, (table_ident, table_builder)) in
            builder.aliases().values().zip(builder.tables().iter())
        {
            if resources.contains_key(alias) {
                log::warn!("Duplicate resource name `{}`", alias);
                continue;
            }
            let relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig> = table_builder
                .relationships
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        BasiliqStoreRelationshipsConfig {
                            target: BasiliqStoreRelationshipTargetConfig {
                                schema: v.ftable_name().schema_name().clone(),
                                table_name: v.ftable_name().table_name().clone(),
                            },
                            enabled: true,
                            field: v.ffield_name().clone(),
                        },
                    )
                })
                .collect();
            resources.insert(
                alias.clone(),
                BasiliqStoreResourceConfig {
                    target: BasiliqStoreRelationshipTargetConfig {
                        schema: table_ident.schema_name().clone(),
                        table_name: table_ident.table_name().clone(),
                    },
                    relationships,
                    enabled: true,
                },
            );
        }
        BasiliqStoreConfig { resources }
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
