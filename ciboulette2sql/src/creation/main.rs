use super::*;

fn insert_relationship_single<'a>(
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    from_type_: &'a CibouletteResourceType,
    to_type_: &'a CibouletteResourceType,
    opt: &bool,
) -> Result<Option<&'a str>, Ciboulette2SqlError> {
    match relationships.get(to_type_.name().as_str()) {
        Some(rel_obj) => match rel_obj.data() {
            Some(CibouletteResourceIdentifierSelector::One(rel_id)) => Ok(Some(rel_id.id())),
            Some(CibouletteResourceIdentifierSelector::Many(_)) => {
                return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                    from_type_.name().to_string(),
                ));
            }
            None => {
                if !opt {
                    return Err(Ciboulette2SqlError::MissingRelationship(
                        from_type_.name().to_string(),
                        to_type_.name().to_string(),
                    ));
                }
                Ok(None)
            }
        },
        None => {
            if !opt {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    from_type_.name().to_string(),
                    to_type_.name().to_string(),
                ));
            }
            Ok(None)
        }
    }
}

pub fn fill_attributes_into_insert_statement<'a>(
    mut stmt: SingleRowInsert<'a>,
    attributes: &'a Option<MessyJsonObjectValue<'a>>,
) -> Result<SingleRowInsert<'a>, Ciboulette2SqlError> {
    if let Some(attributes) = attributes {
        let converted_attr = convert_messy_json_to_str(&attributes)?; // Extract fields
        for (k, v) in converted_attr.into_iter() {
            // Iterate over every attribute
            stmt = stmt.value(quaint::ast::Column::new(k), v); // Insert them in the statement
        }
    }
    Ok(stmt)
}

pub fn query_insert_main<'a>(
    store: &'a CibouletteStore,
    req: &'a CibouletteCreateRequest<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    let main_type = req.path().main_type();
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;

    let returning = crate::utils::extract_sparse(main_type, req.query())?; // Set the returning argument
    let mut insert_stmt = Insert::single_into(main_type.name().to_string()); // Generate base insert statement

    // TODO Pull request `quaint` for `with_capacity` when allocating new single_into
    // TODO Handle client provided id
    // TODO `quaint` should accept Cow
    insert_stmt = fill_attributes_into_insert_statement(insert_stmt, &req.data().attributes())?;
    let mut walker = store
        .graph()
        .neighbors_directed(*main_type_index, petgraph::Direction::Outgoing)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        if let CibouletteRelationshipOption::One(opt) =
            store.graph().edge_weight(edge_index).unwrap()
        // Get the edge weight
        {
            let node_weight = store.graph().node_weight(node_index).unwrap(); // Get the node weight
            let alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
            if let Some(v) = insert_relationship_single(
                &req.data().relationships(),
                &main_type,
                &node_weight,
                opt,
            )? {
                insert_stmt = insert_stmt.value(alias.as_str(), v); // Insert the relationship
            }
        }
    }
    let (query, params) = Postgres::build(insert_stmt.build().returning(returning))?;
    Ok(Ciboulette2SqlRequest {
        request: json_wrap_with_id!(query),
        params: Ciboulette2SqlArguments::new(params),
    })
}
