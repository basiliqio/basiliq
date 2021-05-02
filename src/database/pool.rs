use super::*;
use tracing::info;

/// Open a connection pool to the database provided the connection option
pub async fn get_connection_pool(
    opt: &BasiliqDbConnectionOption,
) -> Result<sqlx::PgPool, sqlx::Error> {
    info!("Connecting to the database..."); // TODO See https://github.com/launchbadge/sqlx/issues/659
    let pool_option = sqlx::pool::PoolOptions::new()
        .max_connections(opt.pool_max_connections().unwrap_or(num_cpus::get()) as u32);
    let res = pool_option
        .connect_with(opt.connection_option().clone())
        .await?;
    info!("Connected"); // TODO See https://github.com/launchbadge/sqlx/issues/659
    Ok(res)
}
