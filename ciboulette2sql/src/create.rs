use super::*;
use messy_json_to_str::convert_messy_json_to_str;
use quaint::ast::Insert;
use quaint::visitor::Postgres;
use quaint::visitor::Visitor;

pub fn ciboulette2sql<'a>(
    store: &'a CibouletteStore,
    req: CibouletteRequest<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    let CibouletteRequest {
        query,
        body,
        intention,
    } = req;
    if body.is_none() {
        return Ok(Ciboulette2SqlRequest::default());
    }
    let CibouletteTopLevel { data, .. } = body.unwrap();
    match data {
        Some(CibouletteResourceSelector::One(data)) => insert_single(store, query, data),
        Some(CibouletteResourceSelector::Many(data)) => {
            unimplemented!()
        }
        None => {
            // Ok(None)
            unimplemented!()
        } //TODO Handle, relationships only
    }
}

fn insert_relationship<'a>(
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

fn insert_relationships<'a>(
    relationships: &BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    from_type_: &'a CibouletteResourceType,
    to_type_: &'a CibouletteResourceType,
    alias: &'a String,
    opt: &'a CibouletteRelationshipBucket,
) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
    match relationships
        .get(alias.as_str())
        .and_then(|x| x.data().as_ref())
    {
        Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            let fields: &[&str; 2] = &[opt.from().as_str(), opt.to().as_str()];
            Ok(vec![Postgres::build(
                Insert::single_into(opt.resource().name())
                    .value(fields[0], "toot")
                    .value(fields[1], rel_id.id().to_string())
                    .build()
                    .returning(fields),
            )?])
        }
        Some(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            let mut values: Vec<Vec<&str>> = Vec::with_capacity(rels_id.len());
            let mut res: Vec<(String, Vec<quaint::ast::Value<'a>>)> =
                Vec::with_capacity(rels_id.len());
            let fields: &[&str; 2] = &[opt.from().as_str(), opt.to().as_str()];
            let mut insert_stmt = Insert::multi_into(opt.resource().name(), fields);
            for rel_id in rels_id.iter() {
                values.push(["toot", rel_id.id().as_ref()].to_vec());
            }
            insert_stmt.values(values).build().returning(fields);
            unimplemented!();
            Ok(res)
        }
        None => Ok(Vec::new()),
    }
}

fn insert_single<'a>(
    store: &'a CibouletteStore,
    query: Option<CibouletteQueryParameters<'a>>,
    data: CibouletteResource<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    let CibouletteResource {
        identifier,
        attributes,
        relationships,
        ..
    } = data;
    let mut res: Vec<(String, Vec<quaint::ast::Value<'a>>)> = Vec::with_capacity(1);
    let mut res_relationships: Vec<(String, Vec<quaint::ast::Value<'_>>)> = Vec::new();

    let (resource_index, resource_type) = utils::extract_type(store, &identifier)?; // Extract type
    let returning = utils::extract_sparse(resource_type, query); // Set the returning argument
    let mut insert_stmt = Insert::single_into(identifier.type_().to_string()); // Generate base insert statement

    // TODO Pull request `quaint` for `with_capacity` when allocating new single_into
    // insert_stmt = insert_stmt.value("id", identifier.id().as_ref()); // Insert the id
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
        let alias: &String = resource_type
            .relationships_type_to_alias()
            .get(node_weight.name())
            .ok_or_else(|| {
                CibouletteError::MissingAliasTranslation(
                    resource_type.name().to_string(),
                    node_weight.name().to_string(),
                )
            })?;

        match edge_weight {
            CibouletteRelationshipOption::One(opt) => {
                if let Some(v) =
                    insert_relationship(&relationships, &resource_type, &node_weight, opt)?
                {
                    insert_stmt = insert_stmt.value(alias.as_str(), v); // Insert the relationship
                }
            }
            CibouletteRelationshipOption::Many(opt) => {
                res_relationships.append(&mut insert_relationships(
                    &relationships,
                    &resource_type,
                    &node_weight,
                    alias,
                    &opt,
                )?);
            }
        }
    }
    res.reserve(res_relationships.len());
    res.push(Postgres::build(insert_stmt.build().returning(returning))?);
    res.append(&mut res_relationships);
    Ok(Ciboulette2SqlRequest { requests: res }) // TODO Find a way to avoid this clone
}
