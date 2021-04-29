use super::*;
use ciboulette::*;
use ciboulette2postgres::*;
use std::convert::TryFrom;

pub mod create;
pub mod delete;
pub mod read;
pub mod update;

async fn exec_query<'request>(
    state: &Arc<BasiliqServerState>,
    inbound_request: &'request dyn CibouletteRequestCommons<'request>,
    query: String,
    params: Ciboulette2SqlArguments<'request>,
) -> Result<Response<Body>, BasiliqServerError> {
    let mut transaction = state.db_pool().begin().await?;

    let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, params)
        .fetch_all(&mut *transaction)
        .await
        .unwrap();
    let rows = Ciboulette2PostgresRow::from_raw(&raw_rows)?;
    let rows_nb = rows.len();
    let response_elements = Ciboulette2PostgresRow::build_response_elements(
        rows,
        state.store().ciboulette(),
        inbound_request.anchor_type(),
        Some(rows_nb),
    )?;
    let accumulator = CibouletteResponseDataBuilder::new(inbound_request, response_elements);
    let response: CibouletteResponse<&serde_json::value::RawValue> = accumulator.build()?;
    let res = Body::from(bytes::Bytes::from(serde_json::to_string(&response)?));
    transaction.commit().await?;
    Ok(Response::builder()
        .header(
            hyper::header::CONTENT_TYPE,
            super::server::JSON_API_CONTENT_TYPE,
        )
        .status(super::status_code::convert_status_code(response.status()))
        .body(res)?)
}

pub async fn handle_request(
    state: &Arc<BasiliqServerState>,
    req: CibouletteRequest<'_>,
) -> Result<Response<Body>, BasiliqServerError> {
    match *req.intention() {
        CibouletteIntention::Create => {
            let create_req = ciboulette::CibouletteCreateRequest::try_from(req)?;
            let (query, params) = create::handle_request(&state, &create_req)?;
            exec_query(state, &create_req, query, params).await
        }
        CibouletteIntention::Read => {
            let read_req = ciboulette::CibouletteReadRequest::try_from(req)?;
            let (query, params) = read::handle_request(&state, &read_req)?;
            exec_query(state, &read_req, query, params).await
        }
        CibouletteIntention::Update => {
            let update_req = ciboulette::CibouletteUpdateRequest::try_from(req)?;
            let (query, params) = update::handle_request(&state, &update_req)?;
            exec_query(state, &update_req, query, params).await
        }
        CibouletteIntention::Delete => {
            let delete_req = ciboulette::CibouletteDeleteRequest::try_from(req)?;
            let (query, params) = delete::handle_request(&state, &delete_req)?;
            exec_query(state, &delete_req, query, params).await
        }
    }
}
