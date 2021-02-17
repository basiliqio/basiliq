use super::*;
use messy_json_to_str::convert_messy_json_to_str;
use quaint::ast::Insert;
use quaint::visitor::Postgres;
use quaint::visitor::Visitor;

const SELECT_ALL: &'static [Cow<'static, str>] = &[Cow::Borrowed("*")];

pub fn ciboulette2sql<'a>(
    store: &'a CibouletteStore,
    req: CibouletteRequest,
) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
    let CibouletteRequest {
        query,
        body,
        intention,
    } = req;
    match body {
        Some(body) => match body.data() {
            Some(CibouletteResourceSelector::One(data)) => {
                // Some(insert_single(store, req.query(), body, data)?)
                unimplemented!()
            }
            Some(CibouletteResourceSelector::Many(data)) => {
                unimplemented!()
            }
            None => {
                // Ok(None)
                unimplemented!()
            } //TODO Handle, relationships only
        },
        None => {
            // Ok(None)
            unimplemented!()
        }
    };
    unimplemented!()
}

fn insert_relationships<'a>(
    relationships: &BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    from_type_: &'a CibouletteResourceType,
    to_type_: &'a CibouletteResourceType,
    opt: &'a CibouletteRelationshipBucket,
) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
    let alias = from_type_
        .relationships_type_to_alias()
        .get(to_type_.name())
        .ok_or_else(|| {
            CibouletteError::MissingAliasTranslation(
                from_type_.name().to_string(),
                to_type_.name().to_string(),
            )
        })?;

    match relationships
        .get(alias.as_str())
        .and_then(|x| x.data().as_ref())
    {
        Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            let res: Vec<(String, Vec<quaint::ast::Value<'a>>)> = Vec::with_capacity(1);
            //TODO:
            unimplemented!();
            Ok(res)
        }
        Some(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            let res: Vec<(String, Vec<quaint::ast::Value<'a>>)> = Vec::with_capacity(1);
            //TODO:
            unimplemented!();
            Ok(res)
        }
        None => Ok(Vec::new()),
    }
}

fn insert_single<'a>(
    store: &'a CibouletteStore,
    query: &'a Option<CibouletteQueryParameters<'a>>,
    data: CibouletteResource<'a>,
) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
    let CibouletteResource {
        identifier,
        attributes,
        relationships,
        ..
    } = data;
    let res: Vec<(String, Vec<quaint::ast::Value<'a>>)> = Vec::with_capacity(1);
    let mut res_relationships: Vec<(String, Vec<quaint::ast::Value<'a>>)> = Vec::new();

    let (resource_index, resource_type) = utils::extract_type(store, &identifier)?; // Extract type
    let returning = utils::extract_sparse(resource_type, query); // Set the returning argument
    let mut insert_stmt = Insert::single_into(identifier.type_().as_ref()); // Generate base insert statement

    // TODO Pull request `quaint` for `with_capacity` when allocating new single_into
    insert_stmt = insert_stmt.value("id", identifier.id().as_ref()); // Insert the id
                                                                     // TODO Customizable `id` field name
    if let Some(attributes) = attributes {
        let converted_attr = convert_messy_json_to_str(attributes)?; // Extract fields
        for (k, v) in converted_attr.into_iter() {
            insert_stmt = insert_stmt.value(quaint::ast::Column::new(k), v);
        }
    }
    let mut walker = store
        .graph()
        .neighbors_directed(resource_index, petgraph::Direction::Outgoing)
        .detach();
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        let edge_weight = store.graph().edge_weight(edge_index).unwrap();
        let node_weight = store.graph().node_weight(node_index).unwrap();

        match edge_weight {
            CibouletteRelationshipOption::One(opt) => {
                match relationships.get(node_weight.name().as_str()) {
                    Some(rel_obj) => {
                        match rel_obj.data() {
                            Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
                                //TODO:
                                unimplemented!();
                            }
                            Some(CibouletteResourceIdentifierSelector::Many(_)) => {
                                return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                                    resource_type.name().to_string(),
                                ));
                            }
                            None => {
                                if !opt {
                                    return Err(Ciboulette2SqlError::MissingRelationship(
                                        resource_type.name().to_string(),
                                        node_weight.name().to_string(),
                                    ));
                                }
                            }
                        }
                    }
                    None => {
                        if !opt {
                            return Err(Ciboulette2SqlError::MissingRelationship(
                                resource_type.name().to_string(),
                                node_weight.name().to_string(),
                            ));
                        }
                    }
                }
            }
            CibouletteRelationshipOption::Many(opt) => {
                res_relationships.append(&mut insert_relationships(
                    &relationships,
                    &resource_type,
                    &node_weight,
                    &opt,
                )?);
            }
        }
    }
    Ok(vec![Postgres::build(
        Insert::from(Insert::single_into(identifier.type_().clone().into_owned()))
            .returning(returning),
    )?]) // TODO Find a way to avoid this clone
}
