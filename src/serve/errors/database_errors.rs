use super::*;
use tracing::error;

/// Handle error on an unknown database error, return a valid HTTP Response
fn handle_db_unknown<'a>(err: sqlx::Error) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    error!("Unknown database error : {:#?}", err);
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::UnknownError.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::UnknownError.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}
/// Handle [database errors](sqlx::Error) return a valid HTTP response
///
/// The HTTP response will be made of the [status code](hyper::StatusCode) and
/// the [ciboulette error object](CibouletteErrorObj)
pub fn handle_db_error<'a>(err: sqlx::Error) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    match &err {
        sqlx::Error::Database(db_err) => match db_err.try_downcast_ref() {
            Some(x) => handle_db_error_code(x).unwrap_or_else(|| handle_db_unknown(err)),
            None => handle_db_unknown(err),
        },
        _ => handle_db_unknown(err),
    }
}

/// Try to convert a database [error code](https://www.postgresql.org/docs/current/errcodes-appendix.html) to a valid
/// HTTP response
///
/// Return None if no conversion was available
fn handle_db_error_code<'a>(
    err: &sqlx::postgres::PgDatabaseError,
) -> Option<(hyper::StatusCode, CibouletteErrorObj<'a>)> {
    match err.code() {
        "unique_violation" => None, // TODO
        _ => None,
    }
}
