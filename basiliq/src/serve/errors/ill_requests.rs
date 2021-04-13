use super::*;

pub fn handle_bad_header<'a>(
    err: BasiliqServerError,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadHeader.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadHeader.title())),
            detail: Some(Cow::Owned(err.to_string())),
            ..Default::default()
        },
    )
}

pub fn handle_bad_content_type<'a>() -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadContentType.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadContentType.title())),
            ..Default::default()
        },
    )
}

pub fn handle_bad_url<'a>(err: url::ParseError) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadUrl.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadUrl.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}
pub fn handle_bad_utf8<'a>(
    err: std::str::Utf8Error,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::Utf8.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::Utf8.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

pub fn handle_bad_json<'a>(err: serde_json::Error) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadJson.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadJson.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

pub fn handle_bad_uuid<'a>(err: uuid::Error) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadUuid.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadUuid.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

pub fn handle_bad_url_encoded<'a>(
    err: serde_urlencoded::de::Error,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadUrlEncoded.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadUrlEncoded.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

pub fn handle_bad_number<'a>(
    err: std::num::ParseIntError,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadNumber.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadNumber.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}
