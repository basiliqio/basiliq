use super::*;
mod normal;
mod relationships;
mod sparse;

pub async fn handle_create<'a>(response: Response<Body>) -> serde_json::Value {
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let res: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    res.as_object().and_then(|x| x.get("data")).unwrap().clone()
}
