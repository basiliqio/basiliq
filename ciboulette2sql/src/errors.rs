use thiserror::Error;

#[derive(Error, Debug)]
pub enum Ciboulette2SqlError {
    // #[error("The json:api type `{0}` is unknown.")]
    // Mi(String),
    #[error("Cannot represent `{0}`")]
    BigDecimal(u128),
    #[error("Missing relationship `{1}` for type `{0}`")]
    MissingRelationship(String, String),
    #[error("The relationship for type `{0}` should have been singular")]
    RequiredSingleRelationship(String),
    #[error(transparent)]
    CibouletteError(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    QuaintError(#[from] quaint::error::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
