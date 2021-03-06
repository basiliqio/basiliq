use super::*;

/// Handle a `POST` request to create a new resource in the database
pub fn handle_request<'request>(
    state: &'request Arc<BasiliqServerState>,
    req: &'request CibouletteCreateRequest<'request>,
) -> Result<(String, Ciboulette2PgArguments<'request>), BasiliqServerError> {
    let req_builder =
        Ciboulette2PgBuilder::gen_insert(state.store().ciboulette(), state.store().tables(), &req)?;
    req_builder.build().map_err(BasiliqServerError::from)
}
