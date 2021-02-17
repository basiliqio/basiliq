use super::*;

const SELECT_ALL: &'static [Cow<'static, str>] = &[Cow::Borrowed("*")];

pub fn extract_sparse<'a>(
    type_: &ciboulette::CibouletteResourceType,
    query: Option<&CibouletteQueryParameters<'a>>,
) -> Vec<String> {
    match query {
        Some(query) => query
            .sparse()
            .get(type_)
            .map(|x| x.as_slice())
            .unwrap_or(SELECT_ALL)
            .to_vec()
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
        None => SELECT_ALL
            .to_vec()
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
    }
}

pub fn extract_type<'a>(
    store: &'a CibouletteStore,
    identifier: &CibouletteResourceIdentifierCreator<'a>,
) -> Result<(petgraph::graph::NodeIndex<u16>, &'a CibouletteResourceType), CibouletteError> {
    store
        .get_type_with_index(identifier.type_().as_ref())
        .ok_or_else(|| CibouletteError::UnknownType(identifier.type_().clone().into_owned()))
}
