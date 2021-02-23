use super::*;

// fn insert_relationships<'a>(
//     relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
//     alias: &'a String,
//     opt: &'a CibouletteRelationshipBucket,
//     main_id: &'a str,
// ) -> Result<Vec<(String, Vec<quaint::ast::Value<'a>>)>, Ciboulette2SqlError> {
//     match relationships
//         .get(alias.as_str())
//         .and_then(|x| x.data().as_ref())
//     {
//         Some(CibouletteResourceIdentifierSelector::One(rel_id)) => {
//             let fields: &[&str; 2] = &[opt.from().as_str(), opt.to().as_str()];
//             Ok(vec![Postgres::build(
//                 Insert::single_into(opt.resource().name())
//                     .value(fields[0], main_id)
//                     .value(fields[1], rel_id.id().to_string())
//                     .build()
//                     .returning(fields),
//             )?])
//         }
//         Some(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
//             let mut values: Vec<Vec<&str>> = Vec::with_capacity(rels_id.len());
//             let mut res: Vec<(String, Vec<quaint::ast::Value<'a>>)> =
//                 Vec::with_capacity(rels_id.len());
//             let fields: &[&str; 2] = &[opt.from().as_str(), opt.to().as_str()];
//             let insert_stmt = Insert::multi_into(opt.resource().name(), fields);
//             for rel_id in rels_id.iter() {
//                 values.push([main_id, rel_id.id().as_ref()].to_vec());
//             }
//             res.push(Postgres::build(
//                 insert_stmt.values(values).build().returning(fields),
//             )?);
//             Ok(res)
//         }
//         None => Ok(Vec::new()),
//     }
// }

// pub fn process_insert_relationships<'a>(
//     store: &'a CibouletteStore,
// 	req: &'a CibouletteCreateRequest<'a>,
// ) -> Result<Vec<Ciboulette2SqlRequest<'a>>, Ciboulette2SqlError> {
//     let mut res: Vec<(String, Vec<quaint::ast::Value<'_>>)> = Vec::new(); // Vector in which the relationships queries will be stored

//     let (resource_index, resource_type) = crate::utils::extract_type(store, &identifier)?; // Extract type
//     let mut walker = store
//         .graph()
//         .neighbors_directed(resource_index, petgraph::Direction::Outgoing)
//         .detach(); // Create a graph walker
//     while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
//         // For each connect edge outgoing from the original node
//         let edge_weight = store.graph().edge_weight(edge_index).unwrap(); // Get the edge weight
//         let node_weight = store.graph().node_weight(node_index).unwrap(); // Get the node weight
//         let alias: &String = resource_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource

//         match edge_weight {
//             CibouletteRelationshipOption::Many(opt) => {
//                 res.append(&mut insert_relationships(
//                     &relationships,
//                     alias,
//                     &opt,
//                     main_id,
//                 )?);
//             }
//             _ => (),
//         }
//     }
//     Ok(Ciboulette2SqlRequest {
//         requests: vec![res],
//     })
// }
