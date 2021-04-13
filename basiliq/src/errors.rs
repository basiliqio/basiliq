use thiserror::Error;

/// # An error throwable by [Basiliq](crate)
#[derive(Error, Debug)]
pub enum BasiliqError {
    #[error(transparent)]
    Ciboulette2Postgres(#[from] ciboulette2postgres::Ciboulette2SqlError),
    #[error(transparent)]
    Ciboulette(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    BasiliqStoreConfigError(#[from] basiliq_database_scanner::BasiliqStoreConfigError),
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
