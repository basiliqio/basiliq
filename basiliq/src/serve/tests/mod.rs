use super::BasiliqServerState;
use basiliq_db_test_proc_macro::basiliq_test;
use basiliq_store::{BasiliqDbScannedTable, BasiliqStore, BasiliqStoreBuilder};
use lazy_static::__Deref;
use std::sync::{Arc, RwLock};
mod errors;
mod requests;
#[macro_use]
mod run_test;
use crate::serve::errors::BasiliqErrorId;
use ciboulette::CibouletteErrorRequest;
use hyper::{Body, Method, Request, Response, StatusCode};
pub use run_test::run_request;

const BASE_URL_TEST_SERVER: &str = "http://myservice.com";

lazy_static::lazy_static! {
    pub static ref BASILIQ_TABLES_CACHE: Arc<RwLock<Option<Vec<Arc<BasiliqDbScannedTable>>>>> =
        Arc::new(RwLock::new(None));
}

async fn get_store(pool: &mut sqlx::PgPool) -> BasiliqStore {
    let mut conn = pool.acquire().await.unwrap();
    let read_lock = BASILIQ_TABLES_CACHE.deref().read().unwrap();
    let tables = match read_lock.deref() {
        Some(x) => x.clone(),
        None => {
            std::mem::drop(read_lock);
            let tables = BasiliqDbScannedTable::scan_db(&mut conn).await.unwrap();
            let mut table_cache = BASILIQ_TABLES_CACHE.deref().write().unwrap();
            *table_cache = Some(tables.clone());
            tables
        }
    };
    let store_builder = BasiliqStoreBuilder::new(tables);
    store_builder.build().unwrap()
}

pub async fn prepare_basiliq_test(mut pool: sqlx::PgPool) -> Arc<BasiliqServerState> {
    let store = get_store(&mut pool).await;
    Arc::new(BasiliqServerState::new(
        pool,
        store,
        trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf().unwrap(),
        url::Url::parse(BASE_URL_TEST_SERVER).unwrap(),
    ))
}

pub fn prepare_basiliq_request(method: Method, uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(format!("{}{}", BASE_URL_TEST_SERVER, uri))
        .body(body)
        .unwrap()
}

pub async fn handle_errors<'a>(
    response: Response<Body>,
    expected_code: BasiliqErrorId,
) -> ciboulette::CibouletteErrorObj<'a> {
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let obj: CibouletteErrorRequest = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(obj.errors().id().as_ref().unwrap(), expected_code.id());
    assert_eq!(
        obj.errors().title().as_ref().unwrap(),
        expected_code.title()
    );
    obj.errors
}

pub fn check_uuid<'store, 'b>(
    value: insta::internals::Content,
    path: insta::internals::ContentPath<'store>,
) -> &'b str {
    if path.to_string().as_str() == ".errors.id" {
        return "[error id]";
    }
    assert_eq!(
        value
            .as_str()
            .unwrap()
            .chars()
            .filter(|&c| c == '-')
            .count(),
        4
    );
    "[uuid]"
}
