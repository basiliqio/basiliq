use super::*;

/// Fails if the array doesn't have a single elements
fn check_len_is_1<'a, T>(table_name: &str, arr: &'a [T]) -> Option<&'a T> {
    match arr.len() {
        0 => warn!("The table {} has no primary key, skipping", table_name),
        1 => return arr.first(),
        _ => warn!(
            "The table {} has too many primary keys, skipping",
            table_name
        ),
    }
    None
}

impl BasiliqStoreBuilder {
    /// Extract the column index of the primary key
    pub fn build_pkeys(table: &BasiliqDbScannedTable) -> BTreeSet<i16> {
        check_len_is_1(table.table().name().as_str(), table.pkeys().as_slice())
            .map(|x| x.columns().iter().copied().collect())
            .unwrap_or_default()
    }

    /// Extract the column indexes of the foreign keys
    pub fn build_fkeys_raw(
        table: &BasiliqDbScannedTable,
    ) -> BTreeMap<i16, (BasiliqStoreTableIdentifier, i16)> {
        let mut res: BTreeMap<i16, (BasiliqStoreTableIdentifier, i16)> = BTreeMap::new();
        for rel in table.fkeys_child() {
            let table_name = name::create_resource_name(&table);
            let lcol = rel
                .lcolumns()
                .as_ref()
                .and_then(|lcol| check_len_is_1(table_name.as_str(), lcol.as_slice()));
            let fcol = rel
                .fcolumns()
                .as_ref()
                .and_then(|fcol| check_len_is_1(table_name.as_str(), fcol.as_slice()));
            if let (Some(lcol), Some(fcol)) = (lcol, fcol) {
                res.insert(
                    *lcol,
                    (
                        BasiliqStoreTableIdentifier {
                            table: rel.ftable().clone(),
                            schema: rel.fschema().clone(),
                        },
                        *fcol,
                    ),
                );
            }
        }
        res
    }
}
