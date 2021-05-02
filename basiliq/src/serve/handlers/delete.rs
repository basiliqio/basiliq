use super::*;

/// Handle a `DELETE` request to delete a resource in the database
pub fn handle_request<'request>(
    state: &'request Arc<BasiliqServerState>,
    req: &'request CibouletteDeleteRequest<'request>,
) -> Result<(String, Ciboulette2PgArguments<'request>), BasiliqServerError> {
    let req_builder =
        Ciboulette2PgBuilder::gen_delete(state.store().ciboulette(), state.store().tables(), &req)?;
    req_builder.build().map_err(BasiliqServerError::from)
}
