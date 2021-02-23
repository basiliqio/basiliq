use super::*;
pub mod main;
pub mod relationships;
use sqlx::{Acquire, Executor, Transaction};

// use main::process_insert_main;
use messy_json_to_str::convert_messy_json_to_str;
use quaint::ast::{Insert, SingleRowInsert};
use quaint::visitor::{Postgres, Visitor};
// use relationships::process_insert_relationships;

pub async fn ciboulette2sql<
    'a,
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + sqlx::Acquire<'a, Database = sqlx::Postgres>,
>(
    conn: E,
    store: &'a CibouletteStore,
    req: &'a CibouletteCreateRequest<'a>,
) -> Result<(), Ciboulette2SqlError> {
    let Ciboulette2SqlRequest {
        request: step1_req,
        params: step1_params,
    } = main::query_insert_main(store, req)?;
    let step1_query = sqlx::query_as_with(step1_req.as_str(), step1_params);

    let mut transactions: Transaction<sqlx::Postgres> = conn.begin().await?;

    println!("Running {:#?}", step1_req);

    let step1_res: Ciboulette2SqlResultWithId = step1_query.fetch_one(&mut transactions).await?;

    println!("{:#?}", step1_res);
    Ok(())
}
