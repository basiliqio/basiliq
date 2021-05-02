use super::*;

/// Handle a `PATCH` request to update a resource from the database
pub fn handle_request<'request>(
    state: &'request Arc<BasiliqServerState>,
    req: &'request CibouletteUpdateRequest<'request>,
) -> Result<(String, Ciboulette2PgArguments<'request>), BasiliqServerError> {
    let req_builder =
        Ciboulette2PgBuilder::gen_update(state.store().ciboulette(), state.store().tables(), &req)?;
    req_builder.build().map_err(BasiliqServerError::from)
}
