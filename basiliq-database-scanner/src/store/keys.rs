use super::*;
use log::warn;

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

impl<'a> BasiliqStoreBuilder<'a> {
    pub fn build_pkeys(table: &BasiliqDbScannerTable) -> Option<i16> {
        check_len_is_1(table.table().name().as_str(), table.pkeys().as_slice())
            .and_then(|x| check_len_is_1(table.table().name().as_str(), x.columns().as_slice()))
            .copied()
    }

    pub fn build_fkeys_raw(table: &BasiliqDbScannerTable) -> BTreeMap<i16, (String, i16)> {
        let mut res: BTreeMap<i16, (String, i16)> = BTreeMap::new();
        for rel in table.fkeys_child() {
            let table_name = name::create_resource_name(&table);
            let ftable_name = name::create_resource_name_from_parts(rel.fschema(), rel.ftable());
            let lcol = rel
                .lcolumns()
                .as_ref()
                .and_then(|lcol| check_len_is_1(table_name.as_str(), lcol.as_slice()));
            let fcol = rel
                .fcolumns()
                .as_ref()
                .and_then(|fcol| check_len_is_1(table_name.as_str(), fcol.as_slice()));
            if let (Some(lcol), Some(fcol)) = (lcol, fcol) {
                res.insert(*lcol, (ftable_name, *fcol));
            }
        }
        res
    }
}
