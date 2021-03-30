use super::*;
use getset::Getters;
use log::warn;
use std::sync::Arc;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqDbScannerTable {
    schema: raw::BasiliqDbScannerSchemaRaw,
    table: raw::BasiliqDbScannerTableRaw,
    pkeys: Vec<raw::BasiliqDbScannerPrimaryKeyRaw>,
    fkeys_child: Vec<raw::BasiliqDbScannerForeignKeyRaw>,
    fkeys_parent: Vec<raw::BasiliqDbScannerForeignKeyRaw>,
    columns_by_name: HashMap<String, Arc<BasiliqDbScannerColumn>>,
    columns_by_id: HashMap<i16, Arc<BasiliqDbScannerColumn>>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqDbScannerColumn {
    column: raw::BasiliqDbScannerColumnRaw,
    type_: BasiliqDbScannerType,
}

#[derive(Debug, Clone)]
pub enum BasiliqDbScannerType {
    Simple(raw::BasiliqDbScannerTypeRaw),
    Nested(raw::BasiliqDbScannerTypeRaw, Box<BasiliqDbScannerType>),
}

impl BasiliqDbScannerTable {
    pub fn new(
        schemas: Vec<raw::BasiliqDbScannerSchemaRaw>,
        tables: Vec<raw::BasiliqDbScannerTableRaw>,
        columns: Vec<raw::BasiliqDbScannerColumnRaw>,
        types: Vec<raw::BasiliqDbScannerTypeRaw>,
        primary_keys: Vec<raw::BasiliqDbScannerPrimaryKeyRaw>,
        foreign_keys: Vec<raw::BasiliqDbScannerForeignKeyRaw>,
    ) -> Vec<Self> {
        let types_map: HashMap<u32, raw::BasiliqDbScannerTypeRaw> =
            types.into_iter().map(|x| (x.id(), x)).collect();
        let columns_grouped: HashMap<u32, Vec<raw::BasiliqDbScannerColumnRaw>> = columns
            .into_iter()
            .map(|x| (x.table(), x))
            .into_group_map_by(|x| x.0)
            .into_iter()
            .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
            .collect();
        let parsed_columns: HashMap<u32, Vec<BasiliqDbScannerColumn>> =
            BasiliqDbScannerColumn::new(&types_map, columns_grouped);
        let mut res: Vec<BasiliqDbScannerTable> = Vec::with_capacity(tables.len());
        let schemas_map: HashMap<u32, raw::BasiliqDbScannerSchemaRaw> =
            schemas.into_iter().map(|x| (x.id(), x)).collect();
        let primary_keys_map: HashMap<u32, Vec<raw::BasiliqDbScannerPrimaryKeyRaw>> = primary_keys
            .into_iter()
            .map(|x| (x.table(), x))
            .into_group_map_by(|x| x.0)
            .into_iter()
            .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
            .collect();
        let foreign_keys_map_child: HashMap<u32, Vec<raw::BasiliqDbScannerForeignKeyRaw>> =
            foreign_keys
                .clone()
                .into_iter()
                .map(|x| (x.table(), x))
                .into_group_map_by(|x| x.0)
                .into_iter()
                .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
                .collect();
        let foreign_keys_map_parent: HashMap<u32, Vec<raw::BasiliqDbScannerForeignKeyRaw>> =
            foreign_keys
                .into_iter()
                .map(|x| (x.ftable(), x))
                .into_group_map_by(|x| x.0)
                .into_iter()
                .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
                .collect();
        for table in tables.into_iter() {
            let columns_store: Vec<Arc<BasiliqDbScannerColumn>> =
                match parsed_columns.get(&table.id()) {
                    Some(x) => x.clone().into_iter().map(Arc::new).collect(),
                    None => {
                        warn!(
                            "Table `{}` doesn't have any columns. Skipping",
                            table.name()
                        );
                        continue;
                    }
                };
            let columns_by_name: HashMap<String, Arc<BasiliqDbScannerColumn>> = columns_store
                .clone()
                .into_iter()
                .map(|x| (x.column().name().clone(), x))
                .collect();
            let columns_by_id: HashMap<i16, Arc<BasiliqDbScannerColumn>> = columns_store
                .clone()
                .into_iter()
                .map(|x| (x.column().column_number(), x))
                .collect();
            let schema = match schemas_map.get(&table.schema()) {
                Some(x) => x.clone(),
                None => {
                    warn!(
                        "Table `{}` is defined in an unknown schema: `{}`",
                        table.name(),
                        table.schema()
                    );
                    continue;
                }
            };
            res.push(BasiliqDbScannerTable {
                schema,
                columns_by_name,
                columns_by_id,
                pkeys: primary_keys_map
                    .get(&table.id())
                    .cloned()
                    .unwrap_or_default(),
                fkeys_child: foreign_keys_map_child
                    .get(&table.id())
                    .cloned()
                    .unwrap_or_default(),
                fkeys_parent: foreign_keys_map_parent
                    .get(&table.id())
                    .cloned()
                    .unwrap_or_default(),
                table,
            })
        }
        res
    }
}

impl BasiliqDbScannerType {
    pub fn new(
        type_: raw::BasiliqDbScannerTypeRaw,
        list_available: &HashMap<u32, raw::BasiliqDbScannerTypeRaw>,
    ) -> Self {
        match type_.child_type() {
            Some(child_id) => {
                let child = match list_available.get(&child_id) {
                    Some(child) => child,
                    None => {
                        warn!("Nested type {} not found for type {}", child_id, type_.id());
                        return BasiliqDbScannerType::Simple(type_); //FIXME
                    }
                };
                BasiliqDbScannerType::Nested(
                    type_,
                    Box::new(BasiliqDbScannerType::new(child.clone(), list_available)),
                )
            }
            None => BasiliqDbScannerType::Simple(type_),
        }
    }
}

impl BasiliqDbScannerColumn {
    pub fn new(
        types: &HashMap<u32, raw::BasiliqDbScannerTypeRaw>,
        columns: HashMap<u32, Vec<raw::BasiliqDbScannerColumnRaw>>,
    ) -> HashMap<u32, Vec<Self>> {
        let mut res: HashMap<u32, Vec<Self>> = HashMap::with_capacity(columns.len());
        for (key, cols) in columns.into_iter() {
            let mut col_list: Vec<Self> = Vec::with_capacity(cols.len());
            for col in cols.into_iter() {
                let type_ = match types.get(&col.type_()) {
                    Some(x) => x.clone(),
                    None => {
                        warn!(
                            "Type `{}` of column `{}` from table `{}` doesn't exists",
                            col.type_(),
                            col.name(),
                            col.table()
                        );
                        continue;
                    }
                };
                col_list.push(BasiliqDbScannerColumn {
                    column: col,
                    type_: BasiliqDbScannerType::new(type_, types),
                });
            }
            res.insert(key, col_list);
        }
        res
    }
}
