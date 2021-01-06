use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostgresSchemaRaw {
    id: u32,
    name: String,
    owner: u32,
    usage: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostgresRoleRaw {
    id: u32,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum PostgresTableType {
	CompositeType = 99, // c
	ForeignTable = 102, // f
	Index = 105, // i
	Ordinary = 114,  // r
	Sequence = 115, // s
	Toast = 116, // t
	View = 118, // v
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostgresTableRaw {
    id: u32,
    name: String,
	schema: u32,
	type_: u32,
	owner: u32,
	kind: PostgresTableType,
	usage_perm: bool,
	select_perm: bool,
	insert_perm: bool,
	update_perm: bool,
	delete_perm: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostgresColumnRaw {
    name: String,
	type_: u32,
	table: u32,
	dimensions: u32,
	non_null: bool,
	has_default: bool,
	insert_perm: bool,
	select_perm: bool,
	update_perm: bool,
	reference_perm: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum PostgresTypeType {
	Base = 98, // b
	Composite = 99, // c
	Domain = 100, // d
	Enum = 101,  // e
	Pseudo = 112, // p
	Range = 114, // r
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i8)]
pub enum PostgresTypeCategory {
	Array = 65, // A
	Boolean = 66, // B
	Composite = 67, // C
	DateTime = 68,  // D
	Enum = 69, // E
	Geo = 71, // G
	NetworkAddress = 73, // I
	Numeric = 78, // N
	Pseudo = 80, // P
	Range = 82, // R
	String = 83, // S
	Timespan = 84, // T
	UserDefined = 85, // U
	BitString = 86, // V
	Unknown = 88, // X
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostgresTypeRaw {
	id: u32,
	name: String,
	schema: u32,
	len: Option<i16>,
	type_: PostgresTypeType,
	rel_id: Option<u32>,
	child_type: Option<u32>,
	parent_type: Option<u32>,
	category: PostgresTypeCategory,
	dimensions: u32
}

pub async fn read_schemas(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresSchemaRaw>> {
    Ok(sqlx::query_file_as_unchecked!(PostgresSchemaRaw, "queries/get_schemas.sql") // Unchecked because of COALESCE
	.fetch_all(db_conn)
	.await?)
}

pub async fn read_tables(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresTableRaw>> {
    Ok(sqlx::query_file_as_unchecked!(PostgresTableRaw, "queries/get_tables.sql") 
	.fetch_all(db_conn)
	.await?)
}

pub async fn read_roles(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresRoleRaw>> {
    Ok(sqlx::query_file_as_unchecked!(PostgresRoleRaw, "queries/get_roles.sql") 
	.fetch_all(db_conn)
	.await?)
}

pub async fn read_columns(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresColumnRaw>> {
    Ok(sqlx::query_file_as_unchecked!(PostgresColumnRaw, "queries/get_columns.sql") 
	.fetch_all(db_conn)
	.await?)
}

pub async fn read_types(db_conn: &sqlx::PgPool) -> Result<Vec<PostgresTypeRaw>> {
    Ok(sqlx::query_file_as_unchecked!(PostgresTypeRaw, "queries/get_types.sql") 
	.fetch_all(db_conn)
	.await?)
}
