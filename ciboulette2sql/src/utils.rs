use super::*;

const SELECT_ALL: &'static [Cow<'static, str>] = &[Cow::Borrowed("*")];

pub fn extract_sparse<'a>(
    type_: &ciboulette::CibouletteResourceType,
    query: &'a Option<CibouletteQueryParameters<'a>>,
) -> std::iter::Map<std::slice::Iter<'a, Cow<'a, str>>, for<'r> fn(&'r Cow<'a, str>) -> &'r str> {
    match query {
        Some(query) => query
            .sparse()
            .get(type_)
            .map(|x| x.as_slice())
            .unwrap_or(SELECT_ALL)
            .iter()
            .map(Cow::as_ref),
        None => SELECT_ALL.iter().map(Cow::as_ref),
    }
}

pub fn extract_type<'a>(
    store: &'a CibouletteStore,
    identifier: &CibouletteResourceIdentifier<'a>,
) -> Result<(petgraph::graph::NodeIndex<u16>, &'a CibouletteResourceType), CibouletteError> {
    store
        .get_type_with_index(identifier.type_().as_ref())
        .ok_or_else(|| CibouletteError::UnknownType(identifier.type_().clone().into_owned()))
}
