use super::*;
use crate::cli::serve::BasiliqCliServerConfig;
use basiliq_database_scanner::BasiliqStore;
use getset::Getters;
use log::info;
use std::{str::FromStr, sync::Arc};

use std::convert::Infallible;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

pub mod addr;
pub mod errors;
pub mod handlers;
pub mod server;

use errors::BasiliqServerError;

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqServerState {
    db_pool: sqlx::PgPool,
    store: BasiliqStore,
    dns_resolver: trust_dns_resolver::TokioAsyncResolver,
    base_url: url::Url,
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
    Ok(BasiliqServerState {
        db_pool: pool,
        store: store_builder.build()?,
        dns_resolver,
        base_url: url::Url::parse("http://localhost:4444/").unwrap(), //FIXME
    })
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
            Ok::<_, BasiliqServerError>(service_fn(move |x| {
                let state = state.clone();
                async move {
                    let res = server::entry_server(state, x).await;
                    println!("{:#?}", res);
                    res
                }
            }))
        }
    });
    Server::bind(&socket_addr).serve(make_svc).await?;
    Ok(())
}
