use tracing::info;

pub async fn get_connection_pool(
    opt: &sqlx::postgres::PgConnectOptions,
) -> Result<sqlx::PgPool, sqlx::Error> {
    info!("Connecting to the database..."); // TODO See https://github.com/launchbadge/sqlx/issues/659
    let res = sqlx::PgPool::connect_with(opt.clone()).await?;
    info!("Connected"); // TODO See https://github.com/launchbadge/sqlx/issues/659
    Ok(res)
}
