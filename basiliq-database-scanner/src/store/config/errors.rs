use super::*;
use thiserror::Error;

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum BasiliqStoreConfigError {
    #[error("Duplicate table entry in the configuration: `{0}`")]
    DuplicateTable(BasiliqStoreTableIdentifier),
    #[error("Unknown table: `{0}`")]
    UnknownTable(BasiliqStoreTableIdentifier),
    #[error("Cannot change target configuration")]
    TargetConfigChange,
}
