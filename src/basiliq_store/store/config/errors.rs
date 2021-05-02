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
    #[error("An unknown resource was found in the {0}, describing the table `{1}`")]
    UnkownResource(BasiliqStoreConfigErrorSource, BasiliqStoreTableIdentifier),
}

/// The source of configuration error
#[derive(Debug, Clone, Copy)]
pub enum BasiliqStoreConfigErrorSource {
    /// The configuration that was built by scanning the database
    BaseConfig,
    /// The configuration that was provided
    ProvidedConfig,
}

impl std::fmt::Display for BasiliqStoreConfigErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasiliqStoreConfigErrorSource::BaseConfig => {
                write!(f, "base configuration")
            }
            BasiliqStoreConfigErrorSource::ProvidedConfig => {
                write!(f, "provided configuration")
            }
        }
    }
}
