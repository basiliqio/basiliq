use super::*;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresSchemaRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get, get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    owner: u32,
    #[getset(get_copy = "pub")]
    usage: bool,
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresRoleRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum PostgresTableType {
    CompositeType = 99, // c
    ForeignTable = 102, // f
    Index = 105,        // i
    Ordinary = 114,     // r
    Sequence = 115,     // s
    Toast = 116,        // t
    View = 118,         // v
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresTableRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    schema: u32,
    #[getset(get_copy = "pub")]
    type_: u32,
    #[getset(get_copy = "pub")]
    owner: u32,
    #[getset(get = "pub")]
    kind: PostgresTableType,
    #[getset(get_copy = "pub")]
    usage_perm: bool,
    #[getset(get_copy = "pub")]
    select_perm: bool,
    #[getset(get_copy = "pub")]
    insert_perm: bool,
    #[getset(get_copy = "pub")]
    update_perm: bool,
    #[getset(get_copy = "pub")]
    delete_perm: bool,
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresColumnRaw {
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    type_: u32,
    #[getset(get_copy = "pub")]
    table: u32,
    #[getset(get_copy = "pub")]
    column_number: i32,
    #[getset(get_copy = "pub")]
    dimensions: u32,
    #[getset(get_copy = "pub")]
    non_null: bool,
    #[getset(get_copy = "pub")]
    has_default: bool,
    #[getset(get_copy = "pub")]
    insert_perm: bool,
    #[getset(get_copy = "pub")]
    select_perm: bool,
    #[getset(get_copy = "pub")]
    update_perm: bool,
    #[getset(get_copy = "pub")]
    reference_perm: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum PostgresTypeType {
    Base = 98,      // b
    Composite = 99, // c
    Domain = 100,   // d
    Enum = 101,     // e
    Pseudo = 112,   // p
    Range = 114,    // r
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum PostgresTypeCategory {
    Array = 65,          // A
    Boolean = 66,        // B
    Composite = 67,      // C
    DateTime = 68,       // D
    Enum = 69,           // E
    Geo = 71,            // G
    NetworkAddress = 73, // I
    Numeric = 78,        // N
    Pseudo = 80,         // P
    Range = 82,          // R
    String = 83,         // S
    Timespan = 84,       // T
    UserDefined = 85,    // U
    BitString = 86,      // V
    Unknown = 88,        // X
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct PostgresTypeRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    schema: u32,
    #[getset(get_copy = "pub")]
    len: Option<i16>,
    #[getset(get = "pub")]
    type_: PostgresTypeType,
    #[getset(get_copy = "pub")]
    rel_id: Option<u32>,
    #[getset(get_copy = "pub")]
    child_type: Option<u32>,
    #[getset(get_copy = "pub")]
    parent_type: Option<u32>,
    #[getset(get = "pub")]
    category: PostgresTypeCategory,
    #[getset(get_copy = "pub")]
    dimensions: u32,
}

pub async fn read_schemas(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresSchemaRaw>> {
    Ok(
        sqlx::query_file_as_unchecked!(PostgresSchemaRaw, "queries/get_schemas.sql") // Unchecked because of COALESCE
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn read_tables(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresTableRaw>> {
    Ok(
        sqlx::query_file_as_unchecked!(PostgresTableRaw, "queries/get_tables.sql")
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn read_roles(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresRoleRaw>> {
    Ok(
        sqlx::query_file_as_unchecked!(PostgresRoleRaw, "queries/get_roles.sql")
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn read_columns(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresColumnRaw>> {
    Ok(
        sqlx::query_file_as_unchecked!(PostgresColumnRaw, "queries/get_columns.sql")
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn read_types(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresTypeRaw>> {
    Ok(
        sqlx::query_file_as_unchecked!(PostgresTypeRaw, "queries/get_types.sql")
            .fetch_all(db_conn)
            .await?,
    )
}
