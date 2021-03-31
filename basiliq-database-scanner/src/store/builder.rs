use super::*;
use ciboulette::CibouletteIdType;
use log::warn;

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
        let mut relationships: BTreeMap<String, BTreeMap<String, (String, i16)>> = BTreeMap::new();

        for table in self.tables.iter() {
            if let Some((table_builder, fkey)) = Self::check_schema(&table)
                .map(|table| (table, Self::build_fkeys_raw(&table)))
                .and_then(|(table, fkeys)| Self::build_pkeys(&table).map(|x| (x, fkeys)))
                .and_then(|(pkey, fkey)| Self::build_object(&table, pkey, &fkey).map(|x| (x, fkey)))
            {
                let nfkey = extract_relationships_fields_name(fkey, &table_builder);
                let name = name::create_resource_name(&table);
                res.insert(name.clone(), table_builder);
                relationships.insert(name, nfkey);
            }
        }
        // for (name, (table_builder, fkeys)) in res.iter() {
        // 	//
        // 	//	Insert a one-to-one in the current table
        // 	//	Insert a one-to-many in the other one
        // 	//
        // 	for (field_name, (other_table_name, field_index)) in fkeys
        // 	{
        // 		if Some(other_table) = res.get_mut(other_table_name)
        // 		{
        // 			other_table
        // 		}
        // 	}
        // 	table_builder.fkeys.insert(key, value)
        // }
        todo!()
    }
}

fn extract_relationships_fields_name(
    fkey: BTreeMap<i16, (String, i16)>,
    table_builder: &BasiliqStoreTableBuilder,
) -> BTreeMap<String, (String, i16)> {
    let mut nfkey: BTreeMap<String, (String, i16)> = BTreeMap::new();
    for (k, v) in fkey.into_iter() {
        if let Some(name) = table_builder
            .table
            .columns_by_id()
            .get(&k)
            .map(|x| x.column().name().as_str())
        {
            nfkey.insert(name.to_string(), v);
        } else {
            warn!(
                "Missing column {} for table `{}`, skipping...",
                k,
                table_builder.table.table().name()
            );
        }
    }
    nfkey
}
