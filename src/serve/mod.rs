use super::*;
use crate::basiliq_store::BasiliqStore;
use crate::cli::serve::BasiliqCliServerConfig;
use getset::Getters;
use std::sync::Arc;
use tracing::{info, log::warn};

use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};

pub mod addr;
pub mod errors;
pub mod handlers;
pub mod server;
pub mod signals;
pub mod status_code;

#[cfg(test)]
mod tests;

use errors::obj::BasiliqServerError;

/// The state of the server that'll be shared with every connection
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct BasiliqServerState {
    /// An handle on the database pool
    db_pool: sqlx::PgPool,
    /// The stored built from the database scan and the configuration
    store: BasiliqStore,
    /// An async DNS resolver initialized from the system configuration
    dns_resolver: trust_dns_resolver::TokioAsyncResolver,
    /// The public URL of this server
    base_url: url::Url,
    /// The server configuration
    config: BasiliqCliServerConfig,
}

impl BasiliqServerState {
    pub fn new(
        db_pool: sqlx::PgPool,
        store: BasiliqStore,
        dns_resolver: trust_dns_resolver::TokioAsyncResolver,
        base_url: url::Url,
        config: BasiliqCliServerConfig,
    ) -> Self {
        BasiliqServerState {
            db_pool,
            store,
            dns_resolver,
            base_url,
            config,
        }
    }
}

/// Build a new server state from the CLI parameters parsing result and the server configuration
pub async fn build_server_state(
    param: &BasiliqCliResult,
    opt: BasiliqCliServerConfig,
) -> Result<BasiliqServerState, BasiliqError> {
    let pool =
        crate::database::pool::get_connection_pool(param.database_connection_infos()).await?;
    let store_builder =
        crate::config::check::create_store_builder_pool(&pool, opt.config_file().clone()).await?;
    let dns_resolver = trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf()?;
    info!("Building store...");
    if opt.demo_mode() {
        warn!("Starting in demo mode !");
    }
    Ok(BasiliqServerState::new(
        pool,
        store_builder.build()?,
        dns_resolver,
        url::Url::parse("http://localhost:4444/").unwrap(),
        opt,
    ))
}
/// Main service of the server
/// Every connection will go through here
pub(crate) async fn main_service(
    state: Arc<BasiliqServerState>,
    req: Request<Body>,
) -> Result<Response<Body>, BasiliqServerError> {
    use tracing::{span, Level};
    let span = span!(Level::TRACE, "connection");
    // Enter the tracing span
    let _entered_span = span.enter();
    // Call the entry function
    let res = server::entry_server(state, req).await;
    let res = match res {
        // If the result is `Ok`, then return it
        Ok(res) => res,
        // Or if the result is an `Err`, convert the error to a ciboulette error
        Err(err) => errors::convert_error_to_body(err)?,
    };
    core::result::Result::<Response<Body>, BasiliqServerError>::Ok(res)
}

/// Start the server with the configuration read from the CLI
pub async fn serve(
    param: &BasiliqCliResult,
    opt: BasiliqCliServerConfig,
) -> Result<(), BasiliqError> {
    // Build the server state
    let state = Arc::new(build_server_state(param, opt).await?);
    // Get the bind address of the server
    let ip_addr = addr::get_bind_address(state.dns_resolver(), &state.config()).await?;
    // Make a new bind socket address with the desired ports
    let socket_addr = std::net::SocketAddr::new(ip_addr, state.config().bind_port());
    // Make a new defautl service
    let make_svc = hyper::service::make_service_fn(|_socket: &AddrStream| {
        // Get a reference to the server state
        let state = state.clone();
        async move {
            Ok::<_, BasiliqServerError>(hyper::service::service_fn(move |req| {
                // Call the main service
                main_service(state.clone(), req)
            }))
        }
    });

    info!("Starting server...");
    Server::bind(&socket_addr)
        .http2_only(false)
        .serve(make_svc)
        .with_graceful_shutdown(signals::wait_for_term_signal())
        .await?;
    Ok(())
}
