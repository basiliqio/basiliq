use lazy_static::lazy_static;
use std::str::FromStr;
use uuid::Uuid;

lazy_static! {
    static ref BASILIQ_DATABASE_URL: String =
        std::env::var("BASILIQ_DATABASE_URL").expect("the database url to be set");
}
fn connect_to_management_pool() -> sqlx::PgPool {
    let num = num_cpus::get();

    sqlx::pool::PoolOptions::new()
        .min_connections(1)
        .max_connections(num as u32)
        .connect_lazy(&BASILIQ_DATABASE_URL)
        .expect("to initialize the management Postgres connection pool")
}
pub async fn init_db() -> (String, sqlx::PgPool) {
    let management_pool = connect_to_management_pool();
    let db_name = format!("basiliq_test_{}", Uuid::new_v4());
    sqlx::query(format!("CREATE DATABASE \"{}\";", db_name.as_str(),).as_str())
        .execute(&management_pool)
        .await
        .expect("to create a new database");
    let conn_opt = sqlx::postgres::PgConnectOptions::from_str(&BASILIQ_DATABASE_URL)
        .expect("to parse the basiliq database url")
        .database(db_name.as_str());
    let pool = sqlx::pool::PoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .connect_lazy_with(conn_opt);
    sqlx::query("CREATE EXTENSION \"uuid-ossp\";")
        .execute(&pool)
        .await
        .expect("to create uuid extension in the database");
    (db_name, pool)
}

pub async fn deinit_db(db_id: String) {
    let management_pool = connect_to_management_pool();
    sqlx::query(format!("DROP DATABASE \"{}\"", db_id.as_str(),).as_str())
        .execute(&management_pool)
        .await
        .expect("to delete a new database");
}
