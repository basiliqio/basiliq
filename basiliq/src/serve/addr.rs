use super::*;

pub async fn get_bind_address(
    dns_resolver: &trust_dns_resolver::TokioAsyncResolver,
    server_cfg: &BasiliqCliServerConfig,
) -> Result<std::net::IpAddr, BasiliqError> {
    dns_resolver
        .lookup_ip(server_cfg.bind_address().as_str())
        .await?
        .iter()
        .next()
        .ok_or(BasiliqError::NoBindableIp)
}
