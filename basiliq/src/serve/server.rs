use super::*;

pub async fn entry_server<'a>(
    state: Arc<BasiliqServerState<'a>>,
    req: Request<Body>,
) -> Result<Response<Body>, BasiliqError> {
    todo!()
}
