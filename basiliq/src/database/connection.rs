use sqlx::ConnectOptions;
use tracing::info;

pub async fn get_single_connection(
    opt: &sqlx::postgres::PgConnectOptions,
) -> Result<sqlx::PgConnection, sqlx::Error> {
    info!("Connecting to the database..."); // TODO See https://github.com/launchbadge/sqlx/issues/659
    let res = opt.connect().await?;
    info!("Connected"); // TODO See https://github.com/launchbadge/sqlx/issues/659
    Ok(res)
}
