use super::*;
use getset::Getters;
use std::rc::Rc;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqTable {
    schema: raw::PostgresSchemaRaw,
    table: raw::PostgresTableRaw,
    pkeys: Vec<raw::PostgresPrimaryKeyRaw>,
    fkeys_child: Vec<raw::PostgresForeignKeyRaw>,
    fkeys_parent: Vec<raw::PostgresForeignKeyRaw>,
    columns_by_name: HashMap<String, Rc<BasiliqColumns>>,
    columns_by_id: HashMap<i16, Rc<BasiliqColumns>>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqColumns {
    column: raw::PostgresColumnRaw,
    type_: BasiliqType,
}

#[derive(Debug, Clone)]
pub enum BasiliqType {
    Simple(raw::PostgresTypeRaw),
    Nested(raw::PostgresTypeRaw, Box<BasiliqType>),
}

impl BasiliqTable {
    pub fn new(
        schemas: Vec<raw::PostgresSchemaRaw>,
        tables: Vec<raw::PostgresTableRaw>,
        columns: Vec<raw::PostgresColumnRaw>,
        types: Vec<raw::PostgresTypeRaw>,
        primary_keys: Vec<raw::PostgresPrimaryKeyRaw>,
        foreign_keys: Vec<raw::PostgresForeignKeyRaw>,
    ) -> Result<Vec<Self>> {
        let types_map: HashMap<u32, raw::PostgresTypeRaw> =
            types.into_iter().map(|x| (x.id(), x)).collect();
        let columns_grouped: HashMap<u32, Vec<raw::PostgresColumnRaw>> = columns
            .into_iter()
            .map(|x| (x.table(), x))
            .into_group_map_by(|x| x.0)
            .into_iter()
            .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
            .collect();
        let parsed_columns: HashMap<u32, Vec<BasiliqColumns>> =
            BasiliqColumns::new(&types_map, columns_grouped)?;
        let mut res: Vec<BasiliqTable> = Vec::with_capacity(tables.len());
        let schemas_map: HashMap<u32, raw::PostgresSchemaRaw> =
            schemas.into_iter().map(|x| (x.id(), x)).collect();
        let primary_keys_map: HashMap<u32, Vec<raw::PostgresPrimaryKeyRaw>> = primary_keys
            .into_iter()
            .map(|x| (x.table(), x))
            .into_group_map_by(|x| x.0)
            .into_iter()
            .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
            .collect();
        let foreign_keys_map_child: HashMap<u32, Vec<raw::PostgresForeignKeyRaw>> = foreign_keys
            .clone()
            .into_iter()
            .map(|x| (x.table(), x))
            .into_group_map_by(|x| x.0)
            .into_iter()
            .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
            .collect();
        let foreign_keys_map_parent: HashMap<u32, Vec<raw::PostgresForeignKeyRaw>> = foreign_keys
            .into_iter()
            .map(|x| (x.ftable(), x))
            .into_group_map_by(|x| x.0)
            .into_iter()
            .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
            .collect();
        for table in tables.into_iter() {
            let columns_store: Vec<Rc<BasiliqColumns>> = parsed_columns
                .get(&table.id())
                .ok_or(anyhow!("No columns defined for the table {}", table.name()))?
                .clone()
                .into_iter()
                .map(|x| Rc::new(x))
                .collect();
            let columns_by_name: HashMap<String, Rc<BasiliqColumns>> = columns_store
                .clone()
                .into_iter()
                .map(|x| (x.column().name().clone(), x))
                .collect();
            let columns_by_id: HashMap<i16, Rc<BasiliqColumns>> = columns_store
                .clone()
                .into_iter()
                .map(|x| (x.column().column_number(), x))
                .collect();
            res.push(BasiliqTable {
                schema: schemas_map
                    .get(&table.schema())
                    .ok_or(anyhow!(
                        "The table {} is defined in an unknown schema : {}",
                        table.name(),
                        table.schema()
                    ))?
                    .clone(),
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
        Ok(res)
    }
}

impl BasiliqType {
    pub fn new(
        type_: raw::PostgresTypeRaw,
        list_available: &HashMap<u32, raw::PostgresTypeRaw>,
    ) -> Result<Self> {
        match type_.child_type() {
            Some(child_id) => {
                let child = list_available.get(&child_id).ok_or(anyhow!(
                    "Nested type {} not found for type {}",
                    child_id,
                    type_.id()
                ))?;
                return Ok(BasiliqType::Nested(
                    type_.clone(),
                    Box::new(BasiliqType::new(child.clone(), list_available)?),
                ));
            }
            None => Ok(BasiliqType::Simple(type_)),
        }
    }
}

impl BasiliqColumns {
    pub fn new(
        types: &HashMap<u32, raw::PostgresTypeRaw>,
        columns: HashMap<u32, Vec<raw::PostgresColumnRaw>>,
    ) -> Result<HashMap<u32, Vec<Self>>> {
        let mut res: HashMap<u32, Vec<Self>> = HashMap::with_capacity(columns.len());
        for (key, cols) in columns.into_iter() {
            let mut col_list: Vec<Self> = Vec::with_capacity(cols.len());
            for col in cols.into_iter() {
                let type_: raw::PostgresTypeRaw = types
                    .get(&col.type_())
                    .ok_or(anyhow!(
                        "The type {} of column {} from table {} doesn't exists",
                        col.type_(),
                        col.name(),
                        col.table()
                    ))
                    .map(|x| x.clone())?;
                col_list.push(BasiliqColumns {
                    column: col,
                    type_: BasiliqType::new(type_, types)?,
                });
            }
            res.insert(key, col_list);
        }
        Ok(res)
    }
}
