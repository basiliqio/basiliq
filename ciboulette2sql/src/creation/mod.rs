use super::*;
mod main;
mod relationships;

use main::process_insert_main;
use messy_json_to_str::convert_messy_json_to_str;
use quaint::ast::{Insert, SingleRowInsert};
use quaint::visitor::{Postgres, Visitor};
use relationships::process_insert_relationships;

pub fn ciboulette2sql<'a>(
    store: &'a CibouletteStore,
    req: &'a CibouletteRequest<'a>,
    step: Ciboulette2SqlStep<'a>,
) -> Result<Ciboulette2SqlRequest<'a>, Ciboulette2SqlError> {
    let CibouletteRequest { query, body, .. } = req;
    match body {
        Some(body) => {
            let CibouletteTopLevel { data, .. } = &body;
            match data {
                Some(CibouletteResourceSelector::One(resource)) => {
                    match step {
                        Ciboulette2SqlStep::Main => process_insert_main(store, &query, resource), // Single insert
                        Ciboulette2SqlStep::Relationships(id) => {
                            process_insert_relationships(store, &query, resource, &id)
                        }
                    }
                }
                Some(CibouletteResourceSelector::Many(resources)) => {
                    // Multiple insert
                    let mut res = Ciboulette2SqlRequest::with_capacity(resources.len());
                    match step {
                        Ciboulette2SqlStep::Main => {
                            for r in resources.iter() {
                                res.append(&mut process_insert_main(store, &query, r)?);
                                // Add as many request as possible
                            }
                        }
                        Ciboulette2SqlStep::Relationships(id) => {
                            for r in resources.iter() {
                                res.append(&mut process_insert_relationships(
                                    store, &query, r, &id,
                                )?); // Add as many request as possible
                            }
                        }
                    }
                    Ok(res)
                }
                None => {
                    Ok(Ciboulette2SqlRequest::default()) // Nothing to do, no main data
                }
            }
        }
        None => return Ok(Ciboulette2SqlRequest::default()), // Not even a body, nothing to do
    }
}
