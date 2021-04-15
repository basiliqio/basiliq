use super::*;
use crate::serve::errors::BasiliqErrorId;
use ciboulette::CibouletteErrorRequest;
use serde_json::json;
mod bad_requests;
mod ciboulette_errors;

use hyper::header::{self, HeaderValue};

pub async fn handle_errors<'a>(
    response: Response<Body>,
    expected_code: BasiliqErrorId,
) -> ciboulette::CibouletteErrorObj<'a> {
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let obj: CibouletteErrorRequest = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(obj.errors().id().as_ref().unwrap(), expected_code.id());
    assert_eq!(
        obj.errors().title().as_ref().unwrap(),
        expected_code.title()
    );
    obj.errors
}
