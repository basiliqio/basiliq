use super::*;
use crate::cli::serve::BasiliqCliServerConfig;
use basiliq_database_scanner::BasiliqStore;
use getset::Getters;
use std::sync::Arc;
use tracing::info;

use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};

pub mod addr;
pub mod errors;
pub mod handlers;
pub mod server;
pub mod status_code;

#[cfg(test)]
mod tests;

use errors::obj::BasiliqServerError;

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqServerState {
    db_pool: sqlx::PgPool,
    store: BasiliqStore,
    dns_resolver: trust_dns_resolver::TokioAsyncResolver,
    base_url: url::Url,
}

impl BasiliqServerState {
    pub fn new(
        db_pool: sqlx::PgPool,
        store: BasiliqStore,
        dns_resolver: trust_dns_resolver::TokioAsyncResolver,
        base_url: url::Url,
    ) -> Self {
        BasiliqServerState {
            db_pool,
            store,
            dns_resolver,
            base_url,
        }
    }
}

pub async fn build_server_state(
    param: &BasiliqCliResult,
    opt: &BasiliqCliServerConfig,
) -> Result<BasiliqServerState, BasiliqError> {
    let pool =
        crate::database::pool::get_connection_pool(param.database_connection_infos()).await?;
    info!("Building store...");
    let store_builder =
        crate::config::check::create_store_builder_pool(&pool, opt.config_file().clone()).await?;
    let dns_resolver = trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf()?;
    Ok(BasiliqServerState::new(
        pool,
        store_builder.build()?,
        dns_resolver,
        url::Url::parse("http://localhost:4444/").unwrap(),
    ))
}

pub(crate) async fn main_service(
    state: Arc<BasiliqServerState>,
    req: Request<Body>,
) -> Result<Response<Body>, BasiliqServerError> {
    use tracing::{span, Level};
    let span = span!(Level::TRACE, "connection");
    let _entered_span = span.enter();
    let res = server::entry_server(state, req).await;
    let res = match res {
        Ok(res) => res,
        Err(err) => errors::convert_error_to_body(err)?,
    };
    core::result::Result::<Response<Body>, BasiliqServerError>::Ok(res)
}

pub async fn serve(
    param: &BasiliqCliResult,
    opt: &BasiliqCliServerConfig,
) -> Result<(), BasiliqError> {
    let state = Arc::new(build_server_state(param, opt).await?);
    let ip_addr = addr::get_bind_address(state.dns_resolver(), &opt).await?;
    let socket_addr = std::net::SocketAddr::new(ip_addr, opt.bind_port());
    let make_svc = hyper::service::make_service_fn(|_socket: &AddrStream| {
        let state = state.clone();

        async move {
            Ok::<_, BasiliqServerError>(hyper::service::service_fn(move |req| {
                main_service(state.clone(), req)
            }))
        }
    });
    Server::bind(&socket_addr)
        .http2_only(false)
        .serve(make_svc)
        .await?;
    Ok(())
}
