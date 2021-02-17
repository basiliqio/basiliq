use super::*;

pub fn query_type_selector<'a>(
    store: &'a CibouletteStore,
    req: CibouletteRequest<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    match req.intention() {
        CibouletteIntention::Create => create::ciboulette2sql(store, req),
        _ => unimplemented!(),
    }
}
