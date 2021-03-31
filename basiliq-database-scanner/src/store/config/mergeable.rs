use super::*;

pub trait BasiliqStoreConfigMergeable {
    fn merge() -> Result<(), BasiliqStoreConfigError>;
}
