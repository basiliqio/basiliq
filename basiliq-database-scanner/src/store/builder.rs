use ciboulette::CibouletteIdType;

use super::*;

#[derive(Debug, Clone)]
pub struct BasiliqStoreBuilder {
    pub(crate) tables: Vec<postgres_metadata::parsed::BasiliqDbScannerTable>,
}

#[derive(Debug, Clone)]
pub struct BasiliqStoreTableBuilder<'a> {
    pub(crate) table: &'a postgres_metadata::parsed::BasiliqDbScannerTable,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) fkeys: BTreeMap<String, (String, CibouletteIdType)>,
    pub(crate) properties: MessyJsonObject<'a>,
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

    pub fn check_schema(table: &BasiliqDbScannerTable) -> Option<&BasiliqDbScannerTable> {
        match POSTGRES_SYSTEM_SCHEMA.contains(&table.schema().name().as_str())
		// If in a system schema
		{
			true => None,
			false => Some(table)
		}
    }

    pub fn build<'a>(&'a self) -> BTreeMap<String, BasiliqStoreTableBuilder<'a>> {
        let mut res: BTreeMap<String, BasiliqStoreTableBuilder<'_>> = BTreeMap::new();

        for table in self.tables.iter() {
            match Self::check_schema(&table)
                .map(|table| (table, Self::build_fkeys_raw(&table)))
                .and_then(|(table, fkeys)| Self::build_pkeys(&table).map(|x| (x, fkeys)))
                .and_then(|(pkey, fkey)| Self::build_object(&table, pkey, fkey))
            {
                Some(table_builder) => res.insert(table.table().name().clone(), table_builder),
                None => None,
            };
        }
        res
    }
}
