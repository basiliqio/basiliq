use super::*;
mod builder_config;
mod errors;
mod mergeable;

pub use errors::{BasiliqStoreConfigError, BasiliqStoreConfigErrorSource};
use itertools::EitherOrBoth;
use itertools::Itertools;
pub use mergeable::BasiliqStoreConfigMergeable;

/// Top level of the Store configuration
///
/// Contains a list of accepted resources
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    Getters,
    MutGetters,
    Default,
)]
#[getset(get = "pub", get_mut = "pub")]
pub struct BasiliqStoreConfig {
    pub(crate) resources: BTreeMap<String, BasiliqStoreResourceConfig>,
}
/// The configuration of a store resource
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters, MutGetters,
)]
#[getset(get = "pub")]
pub struct BasiliqStoreResourceConfig {
    pub(crate) target: BasiliqStoreTableIdentifier,
    #[getset(get_mut = "pub")]
    pub(crate) enabled: bool,
    /// A map of the relationships
    #[getset(get_mut = "pub")]
    pub(crate) relationships: BTreeMap<ArcStr, BasiliqStoreRelationshipsConfig>,
}

impl BasiliqStoreConfigMergeable<BasiliqStoreResourceConfig> for BasiliqStoreResourceConfig {
    fn basiliq_config_merge(
        &mut self,
        other: &BasiliqStoreResourceConfig,
    ) -> Result<(), BasiliqStoreConfigError> {
        if self.target != other.target {
            return Err(BasiliqStoreConfigError::TargetConfigChange);
        }
        let mut new_relationships: BTreeMap<ArcStr, BasiliqStoreRelationshipsConfig> =
            self.relationships.clone();
        self.enabled = other.enabled;

        for x in self
            .relationships()
            .iter()
            .merge_join_by(other.relationships().iter(), |(_k1, v1), (_k2, v2)| {
                v1.target().cmp(v2.target())
            })
        {
            match x {
                EitherOrBoth::Both((k1, v1), (k2, v2)) => {
                    let mut new = v1.clone();
                    new.basiliq_config_merge(v2)?;
                    if k1 != k2 {
                        new_relationships.remove(k1);
                        new_relationships.insert(k2.clone(), new);
                    } else if let Some(x) = new_relationships.get_mut(k1) {
                        *x = new
                    }
                }
                EitherOrBoth::Left((_, v1)) => {
                    return Err(BasiliqStoreConfigError::UnkownResource(
                        BasiliqStoreConfigErrorSource::BaseConfig,
                        v1.target().clone(),
                    ));
                }
                EitherOrBoth::Right((_, v2)) => {
                    return Err(BasiliqStoreConfigError::UnkownResource(
                        BasiliqStoreConfigErrorSource::ProvidedConfig,
                        v2.target().clone(),
                    ));
                }
            }
        }
        self.relationships = new_relationships;
        Ok(())
    }
}

/// The configuration of a store relationships
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipsThroughConfig {
    /// The target table through which the relationship is made
    #[serde(flatten)]
    pub(crate) target: BasiliqStoreTableIdentifier,
    /// The field in the target table through which the relationship is made
    pub(crate) field: ArcStr,
}

/// The configuration of a store relationships
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters, MutGetters,
)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipsConfig {
    /// The target table of this relationship
    pub(crate) target: BasiliqStoreTableIdentifier,
    /// In case of Many-to-Many relationship, the bucket informations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) through: Option<BasiliqStoreRelationshipsThroughConfig>,
    #[getset(get_mut = "pub")]
    pub(crate) enabled: bool,
    /// The field in the target table beeing referenced by that relationship
    pub(crate) field: ArcStr,
}

impl BasiliqStoreConfigMergeable<BasiliqStoreRelationshipsConfig>
    for BasiliqStoreRelationshipsConfig
{
    fn basiliq_config_merge(
        &mut self,
        other: &BasiliqStoreRelationshipsConfig,
    ) -> Result<(), BasiliqStoreConfigError> {
        if self.target != other.target || self.field != other.field {
            return Err(BasiliqStoreConfigError::TargetConfigChange);
        }
        self.enabled = other.enabled;
        Ok(())
    }
}

impl BasiliqStoreConfig {
    /// Check that every resource identifier are unique
    fn check_uniq(&self) -> Result<(), BasiliqStoreConfigError> {
        let mut name_set: BTreeSet<&BasiliqStoreTableIdentifier> = BTreeSet::new();

        for resource in self.resources.values() {
            if !name_set.insert(resource.target()) {
                return Err(BasiliqStoreConfigError::DuplicateTable(
                    resource.target().clone(),
                ));
            }
        }
        Ok(())
    }

    /// Perform logic checks on the store configuration
    pub fn check(&self) -> Result<(), BasiliqStoreConfigError> {
        self.check_uniq()
    }
}
