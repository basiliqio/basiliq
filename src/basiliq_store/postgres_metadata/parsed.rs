use super::*;
use getset::Getters;
use std::sync::Arc;
use tracing::warn;

/// A scanned database from the database
#[derive(Debug, Clone, Getters, PartialEq, Eq)]
#[getset(get = "pub")]
pub struct BasiliqDbScannedTable {
    schema: raw::BasiliqDbScannerSchemaRaw,
    table: raw::BasiliqDbScannerTableRaw,
    pkeys: Vec<raw::BasiliqDbScannerPrimaryKeyRaw>,
    fkeys_child: Vec<raw::BasiliqDbScannerForeignKeyRaw>,
    fkeys_parent: Vec<raw::BasiliqDbScannerForeignKeyRaw>,
    columns_by_name: HashMap<String, Arc<BasiliqDbScannerColumn>>,
    columns_by_id: HashMap<i16, Arc<BasiliqDbScannerColumn>>,
}

/// A scanned column from the database
#[derive(Debug, Clone, Getters, PartialEq, Eq)]
#[getset(get = "pub")]
pub struct BasiliqDbScannerColumn {
    column: raw::BasiliqDbScannerColumnRaw,
    type_: BasiliqDbScannedType,
}

/// The type of scanned type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasiliqDbScannedType {
    /// A normal, simple type
    Simple(raw::BasiliqDbScannerTypeRaw),
    /// A nested type, that wraps another types
    Nested(raw::BasiliqDbScannerTypeRaw, Box<BasiliqDbScannedType>),
}

impl BasiliqDbScannedTable {
    /// Create a new [BasiliqDbScannedTable](BasiliqDbScannedTable) from parts
    // TODO Clean this functions, breaking it into multiple functions
    pub fn new(
        schemas: Vec<raw::BasiliqDbScannerSchemaRaw>,
        tables: Vec<raw::BasiliqDbScannerTableRaw>,
        columns: Vec<raw::BasiliqDbScannerColumnRaw>,
        types: Vec<raw::BasiliqDbScannerTypeRaw>,
        primary_keys: Vec<raw::BasiliqDbScannerPrimaryKeyRaw>,
        foreign_keys: Vec<raw::BasiliqDbScannerForeignKeyRaw>,
    ) -> Vec<Arc<Self>> {
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
        let mut res: Vec<Arc<BasiliqDbScannedTable>> = Vec::with_capacity(tables.len());
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
                .map(|x| (x.table_id(), x))
                .into_group_map_by(|x| x.0)
                .into_iter()
                .map(|(key, vals)| (key, vals.into_iter().map(|(_key, vals)| vals).collect()))
                .collect();
        let foreign_keys_map_parent: HashMap<u32, Vec<raw::BasiliqDbScannerForeignKeyRaw>> =
            foreign_keys
                .into_iter()
                .map(|x| (x.ftable_id(), x))
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
            res.push(Arc::new(BasiliqDbScannedTable {
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
            }))
        }
        res
    }

    /// Scan the database using a single connection, returning a list of [BasiliqDbScannedTable](BasiliqDbScannedTable)
    pub async fn scan_db<'a, I>(conn: I) -> Result<Vec<Arc<Self>>, sqlx::Error>
    where
        I: sqlx::Acquire<'a, Database = sqlx::Postgres, Connection = &'a mut sqlx::PgConnection>,
    {
        let connection = conn.acquire().await?;

        let schemas = raw::read_schemas(&mut *connection).await?;
        let tables = raw::read_tables(&mut *connection).await?;
        let columns = raw::read_columns(&mut *connection).await?;
        let types = raw::read_types(&mut *connection).await?;
        let primary_keys = raw::read_primary_keys(&mut *connection).await?;
        let foreign_keys = raw::read_foreign_keys(&mut *connection).await?;

        Ok(Self::new(
            schemas,
            tables,
            columns,
            types,
            primary_keys,
            foreign_keys,
        ))
    }

    /// Scan the database using a connection pool, returning a list of [BasiliqDbScannedTable](BasiliqDbScannedTable).
    /// It'll try to query the database as concurently as possible
    pub async fn scan_db_pool(pool: sqlx::PgPool) -> Result<Vec<Arc<Self>>, sqlx::Error> {
        let (schemas, tables, columns, types, primary_keys, foreign_keys) = tokio::try_join!(
            raw::read_schemas(&pool),
            raw::read_tables(&pool),
            raw::read_columns(&pool),
            raw::read_types(&pool),
            raw::read_primary_keys(&pool),
            raw::read_foreign_keys(&pool)
        )?;
        Ok(Self::new(
            schemas,
            tables,
            columns,
            types,
            primary_keys,
            foreign_keys,
        ))
    }
}

impl BasiliqDbScannedType {
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
                        return BasiliqDbScannedType::Simple(type_); //FIXME
                    }
                };
                BasiliqDbScannedType::Nested(
                    type_,
                    Box::new(BasiliqDbScannedType::new(child.clone(), list_available)),
                )
            }
            None => BasiliqDbScannedType::Simple(type_),
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
                    type_: BasiliqDbScannedType::new(type_, types),
                });
            }
            res.insert(key, col_list);
        }
        res
    }
}
