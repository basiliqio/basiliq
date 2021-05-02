use thiserror::Error;

/// # An error throwable by [Basiliq](crate)
#[derive(Error, Debug)]
pub enum BasiliqError {
    #[error(transparent)]
    Ciboulette2Pg(#[from] ciboulette2pg::Ciboulette2PgError),
    #[error(transparent)]
    Ciboulette(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    BasiliqStoreConfigError(#[from] crate::basiliq_store::BasiliqStoreConfigError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Dns(#[from] trust_dns_resolver::error::ResolveError),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error("No bindable IP address were found")]
    NoBindableIp,
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
}
