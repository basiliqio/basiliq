use super::*;
use ciboulette2pg::Ciboulette2PgError;
use hyper::StatusCode;

macro_rules! cib2postgres_err {
    ($status:expr, $err_id:ident, $err:expr) => {
        ((
            $status,
            CibouletteErrorObj {
                id: Some(Cow::Borrowed(BasiliqErrorId::$err_id.id())),
                title: Some(Cow::Borrowed(BasiliqErrorId::$err_id.title())),
                detail: Some($err.to_string().into()),
                ..Default::default()
            },
        ))
    };
}

pub fn handle_ciboulette2pg_error<'a>(
    err: Ciboulette2PgError,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    match err {
        Ciboulette2PgError::BigDecimal(_) => {
            cib2postgres_err!(StatusCode::BAD_REQUEST, BadBigNumber, err)
        }
        Ciboulette2PgError::MissingRelationship(_, _) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgMissingRelationship,
            err
        ),
        Ciboulette2PgError::RequiredSingleRelationship(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgRequiredSingleRelationship,
            err
        ),
        Ciboulette2PgError::UnknownTable(_) => {
            cib2postgres_err!(StatusCode::BAD_REQUEST, Ciboulette2PgUnknownTable, err)
        }
        Ciboulette2PgError::EmptyRelValue(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgEmptyRelValueError,
            err
        ),
        Ciboulette2PgError::NullCharIdent(_) => {
            cib2postgres_err!(StatusCode::BAD_REQUEST, Ciboulette2PgNullCharIdent, err)
        }
        Ciboulette2PgError::UpdatingMainObject => {
            cib2postgres_err!(StatusCode::FORBIDDEN, Ciboulette2PgUpdatingMainObject, err)
        }
        Ciboulette2PgError::MultiIdsForSingleRelationships => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgMultiIdsForSingleRelationships,
            err
        ),
        Ciboulette2PgError::ManyRelationshipDirectWrite => cib2postgres_err!(
            StatusCode::FORBIDDEN,
            Ciboulette2PgManyRelationshipDirectWrite,
            err
        ),
        Ciboulette2PgError::MissingRelationForSorting(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgMissingRelationForSorting,
            err
        ),
        Ciboulette2PgError::NonAsciiCharInIdent(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgNonAsciiCharInIdent,
            err
        ),
        Ciboulette2PgError::ProvidedIdOnInserts => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PgProvidedIdOnInserts,
            err
        ),
        Ciboulette2PgError::MissingAttributes => {
            cib2postgres_err!(StatusCode::BAD_REQUEST, Ciboulette2PgMissingAttributes, err)
        }
        Ciboulette2PgError::SortingByMultiRel(_, _) => {
            cib2postgres_err!(StatusCode::FORBIDDEN, Ciboulette2PgSortingByMultiRel, err)
        }
        Ciboulette2PgError::UnknownError => cib2postgres_err!(
            StatusCode::INTERNAL_SERVER_ERROR,
            Ciboulette2PgUnknownError,
            err
        ),
        Ciboulette2PgError::CibouletteError(err) => {
            super::ciboulette_errors::handle_ciboulette_error(err)
        }
        Ciboulette2PgError::BufReaderInto(err) => super::ill_requests::handle_buf_error(err),
        Ciboulette2PgError::Sqlx(_) => {
            cib2postgres_err!(StatusCode::INTERNAL_SERVER_ERROR, BufReaderInto, err)
        }
        Ciboulette2PgError::Io(err) => super::ill_requests::handle_io(err),
        Ciboulette2PgError::Utf8(err) => super::ill_requests::handle_bad_from_utf8(err),
    }
}
