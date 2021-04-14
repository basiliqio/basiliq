use super::*;

pub fn handle_http_error<'a>(err: hyper::Error) -> (hyper::StatusCode, CibouletteErrorObj<'a>) {
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        CibouletteErrorObj {
            id: Some(Cow::Borrowed(BasiliqErrorId::HttpError.id())),
            title: Some(Cow::Borrowed(BasiliqErrorId::HttpError.title())),
            detail: Some(err.to_string().into()),
            ..Default::default()
        },
    )
}
