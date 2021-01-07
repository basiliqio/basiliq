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
    columns_store: Vec<Rc<BasiliqColumns>>,
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
        let parsed_columns: Vec<BasiliqColumns> = BasiliqColumns::new(&types, &columns)?;
        let mut res: Vec<BasiliqTable> = Vec::with_capacity(tables.len());

        for table in tables.iter() {
            let schema_for_table: raw::PostgresSchemaRaw =
                BasiliqTable::find_schema_for_table(&schemas, &tables).ok_or(anyhow!(
                    "The table {} doesn't belong to any schema",
                    table.name()
                ))?;
            let columns_store: Vec<Rc<BasiliqColumns>> =
                BasiliqTable::find_columns_for_table(&table, &parsed_columns)?;
            let columns_by_name: HashMap<String, Rc<BasiliqColumns>> = columns_store
                .iter()
                .map(|x| (x.column().name().clone(), x.clone()))
                .collect();
            let columns_by_id: HashMap<i16, Rc<BasiliqColumns>> = columns_store
                .iter()
                .map(|x| (x.column().column_number(), x.clone()))
                .collect();
            let pkeys: Vec<raw::PostgresPrimaryKeyRaw> =
                BasiliqTable::find_primary_keys_for_table(&table, &primary_keys);
            let (fkeys_parent, fkeys_child): (
                Vec<raw::PostgresForeignKeyRaw>,
                Vec<raw::PostgresForeignKeyRaw>,
            ) = BasiliqTable::find_foreign_keys_for_table(&table, &foreign_keys);
            res.push(BasiliqTable {
                table: table.clone(),
                columns_store,
                columns_by_name,
                columns_by_id,
                pkeys,
                fkeys_parent,
                fkeys_child,
                schema: schema_for_table,
            });
        }
        Ok(res)
    }

    fn find_primary_keys_for_table(
        table: &raw::PostgresTableRaw,
        primary_keys: &Vec<raw::PostgresPrimaryKeyRaw>,
    ) -> Vec<raw::PostgresPrimaryKeyRaw> {
        let mut res: Vec<raw::PostgresPrimaryKeyRaw> = Vec::with_capacity(1);

        for pk in primary_keys.iter() {
            if pk.table() == table.id() {
                res.push(pk.clone());
            }
        }
        res
    }

    fn find_foreign_keys_for_table(
        table: &raw::PostgresTableRaw,
        foreign_keys: &Vec<raw::PostgresForeignKeyRaw>,
    ) -> (
        Vec<raw::PostgresForeignKeyRaw>,
        Vec<raw::PostgresForeignKeyRaw>,
    ) {
        let mut res_parent: Vec<raw::PostgresForeignKeyRaw> = Vec::new();
        let mut res_child: Vec<raw::PostgresForeignKeyRaw> = Vec::new();

        for fk in foreign_keys.iter() {
            if fk.table() == table.id() {
                res_child.push(fk.clone());
            } else if fk.ftable() == table.id() {
                res_parent.push(fk.clone());
            }
        }
        (res_parent, res_child)
    }

    fn find_schema_for_table(
        schemas: &Vec<raw::PostgresSchemaRaw>,
        tables: &Vec<raw::PostgresTableRaw>,
    ) -> Option<raw::PostgresSchemaRaw> {
        for table in tables.iter() {
            for schema in schemas.iter() {
                if schema.id() == table.schema() {
                    return Some(schema.clone());
                }
            }
        }
        None
    }

    fn find_columns_for_table(
        table: &raw::PostgresTableRaw,
        columns: &Vec<BasiliqColumns>,
    ) -> Result<Vec<Rc<BasiliqColumns>>> {
        let mut res: Vec<Rc<BasiliqColumns>> = Vec::with_capacity(columns.len());

        for obj in columns.iter() {
            if obj.column().table() == table.id() {
                res.push(Rc::new(obj.clone()));
            }
        }
        Ok(res)
    }
}

impl BasiliqType {
    pub fn new(
        type_: raw::PostgresTypeRaw,
        list_available: &Vec<raw::PostgresTypeRaw>,
    ) -> Result<Self> {
        match type_.child_type() {
            Some(child_id) => {
                for available_type in list_available.iter() {
                    if available_type.id() == child_id {
                        return Ok(BasiliqType::Nested(
                            type_.clone(),
                            Box::new(BasiliqType::new(type_, list_available)?),
                        ));
                    }
                }
                bail!("Nested type {} not found for type {}", child_id, type_.id());
            }
            None => Ok(BasiliqType::Simple(type_)),
        }
    }
}

impl BasiliqColumns {
    pub fn new(
        types: &Vec<raw::PostgresTypeRaw>,
        columns: &Vec<raw::PostgresColumnRaw>,
    ) -> Result<Vec<Self>> {
        Ok(BasiliqColumns::find_types_for_columns(types, columns)?
            .into_iter()
            .map(|(col, typ)| BasiliqColumns {
                column: col,
                type_: typ,
            })
            .collect())
    }

    fn find_types_for_columns(
        types: &Vec<raw::PostgresTypeRaw>,
        columns: &Vec<raw::PostgresColumnRaw>,
    ) -> Result<Vec<(raw::PostgresColumnRaw, BasiliqType)>> {
        let mut res: Vec<(raw::PostgresColumnRaw, BasiliqType)> = Vec::with_capacity(columns.len());

        for column in columns.iter() {
            for type_ in types.iter() {
                match type_.rel_id() {
                    Some(id) => {
                        if id == column.table() {
                            res.push((column.clone(), BasiliqType::new(type_.clone(), &types)?));
                        }
                    }
                    None => continue,
                }
            }
        }
        Ok(res)
    }
}
