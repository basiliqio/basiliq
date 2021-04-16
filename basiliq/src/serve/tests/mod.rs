use super::BasiliqServerState;
use basiliq_db_test_proc_macro::basiliq_test;
use basiliq_store::{BasiliqDbScannedTable, BasiliqStore, BasiliqStoreBuilder};
use lazy_static::__Deref;
use std::sync::{Arc, RwLock};
mod errors;
mod requests;
use hyper::{Body, Method, Request, Response, StatusCode};

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
