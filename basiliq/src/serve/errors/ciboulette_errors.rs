use super::*;
use ciboulette::CibouletteError;
use hyper::StatusCode;

macro_rules! cib_err {
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

pub fn handle_ciboulette_error<'a>(
    err: CibouletteError,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    match err {
        CibouletteError::SerdeJson(err) => super::ill_requests::handle_bad_json(err),
        CibouletteError::UuidError(err) => super::ill_requests::handle_bad_uuid(err),
        CibouletteError::ParseIntError(err) => super::ill_requests::handle_bad_number(err),
        CibouletteError::SerdeUrlEncoded(err) => super::ill_requests::handle_bad_url_encoded(err),
        CibouletteError::Url(err) => super::ill_requests::handle_bad_url(err),
        CibouletteError::MainTypeClash => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteMainTypeClash, err)
        }
        CibouletteError::UnknownType(_) => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteUnknownType, err)
        }
        CibouletteError::UnknownRelationship(_, _) => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteUnknownRelationship, err)
        }
        CibouletteError::UnknownField(_, _) => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteUnknownField, err)
        }
        CibouletteError::NestedSorting => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteNestedSorting, err)
        }
        CibouletteError::NoData => cib_err!(StatusCode::BAD_REQUEST, CibouletteNoData, err),
        CibouletteError::MissingId => cib_err!(StatusCode::BAD_REQUEST, CibouletteMissingId, err),
        CibouletteError::NoCompound => cib_err!(StatusCode::BAD_REQUEST, CibouletteNoCompound, err),
        CibouletteError::MissingTypeInPath => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteMissingTypeInPath, err)
        }
        CibouletteError::BadPath => cib_err!(StatusCode::BAD_REQUEST, CibouletteBadPath, err),
        CibouletteError::WrongPathType(_, _) => {
            cib_err!(StatusCode::BAD_REQUEST, CibouletteWrongPathType, err)
        }
        CibouletteError::UnknownError(_) => {
            cib_err!(
                StatusCode::INTERNAL_SERVER_ERROR,
                CibouletteUnknownError,
                err
            )
        }
        _ => {
            // FIXME Add a way to get more insight
            cib_err!(
                StatusCode::INTERNAL_SERVER_ERROR,
                CibouletteUnknownError,
                err
            )
        }
    }
}
