use super::*;

pub fn convert_error_to_body(
    err: BasiliqServerError,
) -> Result<Response<Body>, BasiliqServerError> {
    let (status, mut err_obj) = match err {
        BasiliqServerError::BadContentType => super::ill_requests::handle_bad_content_type(),
        BasiliqServerError::BadHeader(_, _) => super::ill_requests::handle_bad_header(err),
        BasiliqServerError::BadUrl(err) => super::ill_requests::handle_bad_url(err),
        BasiliqServerError::Utf8(err) => super::ill_requests::handle_bad_utf8(err),
        BasiliqServerError::Json(err) => super::ill_requests::handle_bad_json(err),
        BasiliqServerError::CibouletteError(err) => {
            super::ciboulette_errors::handle_ciboulette_error(err)
        }
        _ => unimplemented!(),
    };

    *err_obj.status_mut() = status.as_u16() as u64;
    let err_response = CibouletteErrorRequest::new(err_obj, None);
    let body = Body::from(bytes::Bytes::from(serde_json::to_string(&err_response)?));

    Ok(Response::builder()
        .header(
            hyper::header::CONTENT_TYPE,
            crate::serve::server::JSON_API_CONTENT_TYPE,
        )
        .status(status)
        .body(body)?)
}
