use super::*;
use ciboulette::CibouletteIntention;

fn check_content_type(req: &Request<Body>) -> Result<(), BasiliqServerError> {
    match req.headers().get(hyper::header::CONTENT_TYPE) {
        Some(content_type) => match content_type
            .to_str()
            .map_err(|err| BasiliqServerError::BadHeader(hyper::header::CONTENT_TYPE, err))?
            == "application/vnd.api+json"
        {
            true => Ok(()),
            false => Err(BasiliqServerError::BadContentType),
        },
        None => Ok(()),
    }
}

fn get_ciboulette_intention(
    req: &Request<Body>,
) -> Result<ciboulette::CibouletteIntention, BasiliqServerError> {
    match *req.method() {
        hyper::Method::GET => Ok(CibouletteIntention::Read),
        hyper::Method::PATCH => Ok(CibouletteIntention::Update),
        hyper::Method::DELETE => Ok(CibouletteIntention::Delete),
        hyper::Method::POST => Ok(CibouletteIntention::Create),
        hyper::Method::OPTIONS => todo!(),
        _ => Err(BasiliqServerError::BadMethod(req.method().clone())),
    }
}

pub async fn entry_server(
    state: Arc<BasiliqServerState>,
    req: Request<Body>,
) -> Result<Response<Body>, BasiliqServerError> {
    info!("{} {}", req.method(), req.uri().path());
    check_content_type(&req)?;
    let intention = get_ciboulette_intention(&req)?;
    let req_url = state.base_url().join(
        req.uri()
            .path_and_query()
            .map(|x| x.as_str())
            .unwrap_or_default(),
    )?;
    let body = hyper::body::to_bytes(req.into_body()).await?;
    let body_str = Some(std::str::from_utf8(&body)?);
    let ciboulette_request_builder = ciboulette::CibouletteInboundRequestBuilder::new(
        intention,
        &req_url,
        match body.is_empty() {
            true => &None,
            false => &body_str,
        },
    );
    let state_copy = state.clone();
    let ciboulette_request = ciboulette_request_builder.build(&state_copy.store().ciboulette())?;
    handlers::handle_request(&state, ciboulette_request).await
}
