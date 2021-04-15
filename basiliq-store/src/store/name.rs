use super::*;

/// Create a default resource name
pub(super) fn create_resource_name(table: &BasiliqDbScannedTable) -> String {
    create_resource_name_from_parts(table.schema().name(), table.table().name())
}

pub(super) fn create_resource_name_from_parts(schema: &str, table: &str) -> String {
    format!("{}__{}", schema, table)
}
