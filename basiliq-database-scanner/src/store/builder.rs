use super::*;

#[derive(Debug, Clone)]
pub struct BasiliqStoreBuilder {
    pub(crate) tables: Vec<postgres_metadata::parsed::BasiliqDbScannerTable>,
}

impl BasiliqStoreBuilder {
    pub async fn scan_db(pool: sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let (schemas, tables, columns, types, primary_keys, foreign_keys) = tokio::try_join!(
            postgres_metadata::raw::read_schemas(&pool),
            postgres_metadata::raw::read_tables(&pool),
            postgres_metadata::raw::read_columns(&pool),
            postgres_metadata::raw::read_types(&pool),
            postgres_metadata::raw::read_primary_keys(&pool),
            postgres_metadata::raw::read_foreign_keys(&pool)
        )?;
        Ok(BasiliqStoreBuilder {
            tables: postgres_metadata::parsed::BasiliqDbScannerTable::new(
                schemas,
                tables,
                columns,
                types,
                primary_keys,
                foreign_keys,
            ),
        })
    }
}
