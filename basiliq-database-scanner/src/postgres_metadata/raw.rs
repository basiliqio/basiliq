use super::*;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct BasiliqDbScannerSchemaRaw {
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
pub struct BasiliqDbScannerRoleRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum BasiliqDbScannerTableType {
    CompositeType = 99, // c
    ForeignTable = 102, // f
    Index = 105,        // i
    Ordinary = 114,     // r
    Sequence = 115,     // s
    Toast = 116,        // t
    View = 118,         // v
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct BasiliqDbScannerTableRaw {
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
    kind: BasiliqDbScannerTableType,
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
pub struct BasiliqDbScannerColumnRaw {
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    table: u32,
    #[getset(get_copy = "pub")]
    column_number: i16,
    #[getset(get_copy = "pub")]
    type_: u32,
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
pub enum BasiliqDbScannerTypeType {
    Base = 98,      // b
    Composite = 99, // c
    Domain = 100,   // d
    Enum = 101,     // e
    Pseudo = 112,   // p
    Range = 114,    // r
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum BasiliqDbScannerTypeCategory {
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
pub struct BasiliqDbScannerTypeRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    schema: u32,
    #[getset(get_copy = "pub")]
    len: Option<i16>,
    #[getset(get = "pub")]
    type_: BasiliqDbScannerTypeType,
    #[getset(get_copy = "pub")]
    rel_id: Option<u32>,
    #[getset(get_copy = "pub")]
    child_type: Option<u32>,
    #[getset(get_copy = "pub")]
    parent_type: Option<u32>,
    #[getset(get = "pub")]
    category: BasiliqDbScannerTypeCategory,
    #[getset(get_copy = "pub")]
    dimensions: u32,
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct BasiliqDbScannerPrimaryKeyRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    schema: u32,
    #[getset(get_copy = "pub")]
    table: u32,
    #[getset(get_copy = "pub")]
    index: u32,
    #[getset(get = "pub")]
    columns: Vec<i16>,
}

#[derive(Getters, CopyGetters, Clone, Debug, Serialize, Deserialize)]
pub struct BasiliqDbScannerForeignKeyRaw {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    schema: u32,
    #[getset(get_copy = "pub")]
    table: u32,
    #[getset(get_copy = "pub")]
    index: u32,
    #[getset(get_copy = "pub")]
    ftable: u32,
    #[getset(get = "pub")]
    lcolumns: Option<Vec<i16>>,
    #[getset(get = "pub")]
    fcolumns: Option<Vec<i16>>,
}

pub async fn read_schemas<'a, E>(db_conn: E) -> Result<Vec<BasiliqDbScannerSchemaRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(sqlx::query_file_as_unchecked!(
        BasiliqDbScannerSchemaRaw,
        "discovery_queries/get_schemas.sql"
    ) // Unchecked because of COALESCE
    .fetch_all(db_conn)
    .await?)
}

pub async fn read_tables<'a, E>(db_conn: E) -> Result<Vec<BasiliqDbScannerTableRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(
        sqlx::query_file_as_unchecked!(
            BasiliqDbScannerTableRaw,
            "discovery_queries/get_tables.sql"
        )
        .fetch_all(db_conn)
        .await?,
    )
}

pub async fn read_roles<'a, E>(db_conn: E) -> Result<Vec<BasiliqDbScannerRoleRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(
        sqlx::query_file_as_unchecked!(BasiliqDbScannerRoleRaw, "discovery_queries/get_roles.sql")
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn read_columns<'a, E>(db_conn: E) -> Result<Vec<BasiliqDbScannerColumnRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(sqlx::query_file_as_unchecked!(
        BasiliqDbScannerColumnRaw,
        "discovery_queries/get_columns.sql"
    )
    .fetch_all(db_conn)
    .await?)
}

pub async fn read_types<'a, E>(db_conn: E) -> Result<Vec<BasiliqDbScannerTypeRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(
        sqlx::query_file_as_unchecked!(BasiliqDbScannerTypeRaw, "discovery_queries/get_types.sql")
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn read_primary_keys<'a, E>(
    db_conn: E,
) -> Result<Vec<BasiliqDbScannerPrimaryKeyRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(sqlx::query_file_as_unchecked!(
        BasiliqDbScannerPrimaryKeyRaw,
        "discovery_queries/get_primary_keys.sql"
    )
    .fetch_all(db_conn)
    .await?)
}
pub async fn read_foreign_keys<'a, E>(
    db_conn: E,
) -> Result<Vec<BasiliqDbScannerForeignKeyRaw>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(sqlx::query_file_as!(
        BasiliqDbScannerForeignKeyRaw,
        "discovery_queries/get_foreign_keys.sql"
    )
    .fetch_all(db_conn)
    .await?)
}
