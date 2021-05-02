use super::*;

/// Create a default resource name
pub(super) fn create_resource_name(table: &BasiliqDbScannedTable) -> String {
    create_resource_name_from_parts(table.schema().name(), table.table().name())
}

/// Create a resource name from a schema name and a table name
pub(super) fn create_resource_name_from_parts(schema: &str, table: &str) -> String {
    format!("{}__{}", schema, table)
}
