use super::*;

pub trait BasiliqStoreConfigMergeable {
    fn basiliq_config_merge(&mut self, other: &Self) -> Result<(), BasiliqStoreConfigError>;
}
