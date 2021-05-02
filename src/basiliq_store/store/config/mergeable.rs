use super::*;

/// Trait for mergeable configuration object
pub trait BasiliqStoreConfigMergeable<T> {
    /// Merge another configuration object with self
    fn basiliq_config_merge(&mut self, other: &T) -> Result<(), BasiliqStoreConfigError>;
}
