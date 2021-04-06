use super::*;
use thiserror::Error;

/// # An error throwable by [Basiliq](crate)
#[derive(Error, Debug)]
pub enum BasiliqError {
    #[error(transparent)]
    Ciboulette2Postgres(#[from] ciboulette2postgres::Ciboulette2SqlError),
    #[error(transparent)]
    Ciboulette(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    SQLx(#[from] sqlx::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
}
