use super::*;
use thiserror::Error;

/// # An error throwable by [OApi](crate)
#[derive(Error, Debug)]
pub enum BasiliqStoreConfigError {
    #[error("Duplicate table entry in the configuration: `{0}`")]
    DuplicateTable(BasiliqStoreRelationshipTargetConfig),
    #[error("Unknown table: `{0}`")]
    UnknownTable(BasiliqStoreRelationshipTargetConfig),
}
