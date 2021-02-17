use super::*;

fn insert_relationship_single<'a>(
    relationships: &BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    from_type_: &'a CibouletteResourceType,
    to_type_: &'a CibouletteResourceType,
    opt: &bool,
) -> Result<Option<String>, Ciboulette2SqlError> {
    match relationships.get(to_type_.name().as_str()) {
        Some(rel_obj) => match rel_obj.data() {
            Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
                Ok(Some(rel_id.id().to_string()))
            }
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

pub fn process_insert_main<'a>(
    store: &'a CibouletteStore,
    query: &'a Option<CibouletteQueryParameters<'a>>,
    data: &'a CibouletteResource<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    let CibouletteResource {
        identifier,
        attributes,
        relationships,
        ..
    } = data; // Extract the necessary attributes
    let mut res: Vec<(String, Vec<quaint::ast::Value<'a>>)> = Vec::with_capacity(1); // Vector in which the main query will be stored

    let (resource_index, resource_type) = crate::utils::extract_type(store, &identifier)?; // Extract type
    let returning = crate::utils::extract_sparse(resource_type, query.as_ref()); // Set the returning argument
    let mut insert_stmt = Insert::single_into(identifier.type_().to_string()); // Generate base insert statement

    // TODO Pull request `quaint` for `with_capacity` when allocating new single_into
    // TODO Handle client provided id
    insert_stmt = fill_attributes_into_insert_statement(insert_stmt, &attributes)?;
    let mut walker = store
        .graph()
        .neighbors_directed(resource_index, petgraph::Direction::Outgoing)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        let edge_weight = store.graph().edge_weight(edge_index).unwrap(); // Get the edge weight
        let node_weight = store.graph().node_weight(node_index).unwrap(); // Get the node weight
        let alias: &String = resource_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource

        match edge_weight {
            CibouletteRelationshipOption::One(opt) => {
                if let Some(v) =
                    insert_relationship_single(&relationships, &resource_type, &node_weight, opt)?
                {
                    insert_stmt = insert_stmt.value(alias.as_str(), v); // Insert the relationship
                }
            }
            _ => (),
        }
    }
    res.push(Postgres::build(insert_stmt.build().returning(returning))?);
    Ok(Ciboulette2SqlRequest {
        requests: vec![res],
    })
}
