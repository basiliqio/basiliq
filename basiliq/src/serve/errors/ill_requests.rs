use super::*;

/// Return an error HTTP response when receiving a request with a bad header
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

/// Return an error HTTP response when receiving a bad content type
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

/// Return an error HTTP response when receiving a bad url
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

/// Return an error HTTP response when encountering a bad UTF-8 string (output)
pub fn handle_bad_to_utf8<'a>(
    err: std::str::Utf8Error,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::ToUtf8.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::ToUtf8.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

/// Return an error HTTP response when encountering a bad UTF-8 string (input)
pub fn handle_bad_from_utf8<'a>(
    err: std::string::FromUtf8Error,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::FromUtf8.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::FromUtf8.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

/// Return an error HTTP response when encountering a bad JSON
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

/// Return an error HTTP response when encountering a bad UUID
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

/// Return an error HTTP response when encountering a bad url-encoded string
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

/// Return an error HTTP response when failing to parse a number
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

/// Return an error HTTP response on IO failure
pub fn handle_io<'a>(err: std::io::Error) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::Io.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::Io.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

/// Return an error HTTP response on [buf_redux error](buf_redux) failure
pub fn handle_buf_error<'a>(
    err: buf_redux::IntoInnerError<buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>>,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::Io.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::Io.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

/// Return an error HTTP response on a bad request
pub fn handle_bad_request<'a>(
    err: hyper::http::Error,
) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::BAD_REQUEST,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadRequest.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadRequest.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}

/// Return an error HTTP response on a bad method
pub fn handle_bad_method<'a>() -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::METHOD_NOT_ALLOWED,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::BadMethod.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::BadMethod.title())),
            ..Default::default()
        },
    )
}
