use super::*;
use ciboulette2postgres::Ciboulette2SqlError;
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

pub fn handle_ciboulette2postgres_error<'a>(
    err: Ciboulette2SqlError,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    match err {
        Ciboulette2SqlError::BigDecimal(_) => {
            cib2postgres_err!(StatusCode::BAD_REQUEST, BadBigNumber, err)
        }
        Ciboulette2SqlError::MissingRelationship(_, _) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresMissingRelationship,
            err
        ),
        Ciboulette2SqlError::RequiredSingleRelationship(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresRequiredSingleRelationship,
            err
        ),
        Ciboulette2SqlError::UnknownTable(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresUnknownTable,
            err
        ),
        Ciboulette2SqlError::EmptyRelValue(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresEmptyRelValueError,
            err
        ),
        Ciboulette2SqlError::NullCharIdent(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresNullCharIdent,
            err
        ),
        Ciboulette2SqlError::UpdatingMainObject => cib2postgres_err!(
            StatusCode::FORBIDDEN,
            Ciboulette2PostgresUpdatingMainObject,
            err
        ),
        Ciboulette2SqlError::MultiIdsForSingleRelationships => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresMultiIdsForSingleRelationships,
            err
        ),
        Ciboulette2SqlError::ManyRelationshipDirectWrite => cib2postgres_err!(
            StatusCode::FORBIDDEN,
            Ciboulette2PostgresManyRelationshipDirectWrite,
            err
        ),
        Ciboulette2SqlError::MissingRelationForSorting(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresMissingRelationForSorting,
            err
        ),
        Ciboulette2SqlError::NonAsciiCharInIdent(_) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresNonAsciiCharInIdent,
            err
        ),
        Ciboulette2SqlError::ProvidedIdOnInserts => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresProvidedIdOnInserts,
            err
        ),
        Ciboulette2SqlError::MissingAttributes => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresMissingAttributes,
            err
        ),
        Ciboulette2SqlError::SortingByMultiRel(_, _) => cib2postgres_err!(
            StatusCode::BAD_REQUEST,
            Ciboulette2PostgresSortingByMultiRel,
            err
        ),
        Ciboulette2SqlError::UnknownError => cib2postgres_err!(
            StatusCode::INTERNAL_SERVER_ERROR,
            Ciboulette2PostgresUnknownError,
            err
        ),
        Ciboulette2SqlError::CibouletteError(err) => {
            super::ciboulette_errors::handle_ciboulette_error(err)
        }
        Ciboulette2SqlError::BufReaderInto(err) => super::ill_requests::handle_buf_error(err),
        Ciboulette2SqlError::Sqlx(_) => {
            cib2postgres_err!(StatusCode::INTERNAL_SERVER_ERROR, BufReaderInto, err)
        }
        Ciboulette2SqlError::Io(err) => super::ill_requests::handle_io(err),
        Ciboulette2SqlError::Utf8(err) => super::ill_requests::handle_bad_from_utf8(err),
    }
}
