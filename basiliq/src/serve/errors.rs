use thiserror::Error;

/// # An error throwable by [Basiliq](crate)
#[derive(Error, Debug)]
pub enum BasiliqServerError {
    #[error("The only accepted content type is `application/vnd.api+json`")]
    BadContentType,
    #[error("Impossible to read header `{0}`: {1}")]
    BadHeader(hyper::header::HeaderName, hyper::header::ToStrError),
    #[error("Unsupported method `{0}`")]
    BadMethod(hyper::Method),
    #[error(transparent)]
    BadUrl(#[from] url::ParseError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    UTF8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    CibouletteError(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    Ciboulette2PostgresError(#[from] ciboulette2postgres::Ciboulette2SqlError),
}
