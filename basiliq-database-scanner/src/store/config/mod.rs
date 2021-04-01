use super::*;
mod convert;
mod errors;
mod mergeable;
pub use errors::BasiliqStoreConfigError;
use itertools::EitherOrBoth;
use itertools::Itertools;
pub use mergeable::BasiliqStoreConfigMergeable;

/// Top level of the Store configuration
///
/// Contains a list of accepted resources
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreConfig {
    pub(crate) resources: BTreeMap<String, BasiliqStoreResourceConfig>,
}

impl BasiliqStoreConfigMergeable<BasiliqStoreConfig> for BasiliqStoreConfig {
    fn basiliq_config_merge(&mut self, other: &Self) -> Result<(), BasiliqStoreConfigError> {
        let mut new_resources: BTreeMap<String, BasiliqStoreResourceConfig> =
            self.resources.clone();

        for x in self
            .resources()
            .iter()
            .merge_join_by(other.resources().iter(), |(_k1, v1), (_k2, v2)| {
                v1.target().cmp(v2.target())
            })
        {
            match x {
                EitherOrBoth::Both((k1, v1), (k2, v2)) => {
                    if k1 != k2 {
                        let mut new = v1.clone();
                        new.basiliq_config_merge(v2)?;
                        new_resources.remove(k1);
                        new_resources.insert(k2.clone(), new);
                    }
                }
                EitherOrBoth::Left((k1, _v1)) => {
                    new_resources.remove(k1);
                }
                EitherOrBoth::Right((k2, v2)) => {
                    new_resources.insert(k2.clone(), v2.clone());
                }
            }
        }
        Ok(())
    }
}

/// The configuration of a store resource
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreResourceConfig {
    pub(crate) target: BasiliqStoreTableIdentifier,
    pub(crate) enabled: bool,
    /// A map of the relationships
    pub(crate) relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig>,
}

impl BasiliqStoreConfigMergeable<BasiliqStoreResourceConfig> for BasiliqStoreResourceConfig {
    fn basiliq_config_merge(
        &mut self,
        other: &BasiliqStoreResourceConfig,
    ) -> Result<(), BasiliqStoreConfigError> {
        if self.target != other.target {
            return Err(BasiliqStoreConfigError::TargetConfigChange);
        }
        let mut new_relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig> =
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
                    if k1 != k2 {
                        let mut new = v1.clone();
                        new.basiliq_config_merge(v2)?;
                        new_relationships.remove(k1);
                        new_relationships.insert(k2.clone(), new);
                    }
                }
                EitherOrBoth::Left((k1, _v1)) => {
                    new_relationships.remove(k1);
                }
                EitherOrBoth::Right((k2, v2)) => {
                    new_relationships.insert(k2.clone(), v2.clone());
                }
            }
        }
        Ok(())
    }
}

/// The configuration of a store relationships
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreRelationshipsConfig {
    pub(crate) target: BasiliqStoreTableIdentifier,
    pub(crate) enabled: bool,
    pub(crate) field: String,
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
    pub fn check(&self) -> Result<(), BasiliqStoreConfigError> {
        self.check_uniq()
    }
}
