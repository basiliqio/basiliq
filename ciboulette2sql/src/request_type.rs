use super::*;

pub fn query_type_selector<'a>(
    store: &'a CibouletteStore,
    req: &'a CibouletteRequest<'a>,
    step: Ciboulette2SqlStep<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    match req.intention() {
        CibouletteIntention::Create => creation::ciboulette2sql(store, req, step),
        _ => unimplemented!(),
    }
}
