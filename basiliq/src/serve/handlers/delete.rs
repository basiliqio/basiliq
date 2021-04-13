use super::*;

pub fn handle_request<'request>(
    state: &'request Arc<BasiliqServerState>,
    req: &'request CibouletteDeleteRequest<'request>,
) -> Result<(String, Ciboulette2SqlArguments<'request>), BasiliqServerError> {
    let req_builder = Ciboulette2PostgresBuilder::gen_delete(
        state.store().ciboulette(),
        state.store().tables(),
        &req,
    )?;
    req_builder.build().map_err(BasiliqServerError::from)
}