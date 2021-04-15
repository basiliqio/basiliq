use super::*;

pub trait BasiliqStoreConfigMergeable<T> {
    fn basiliq_config_merge(&mut self, other: &T) -> Result<(), BasiliqStoreConfigError>;
}
