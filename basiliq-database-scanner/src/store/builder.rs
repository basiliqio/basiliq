use super::*;
use ciboulette::CibouletteIdType;
use log::warn;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BasiliqStoreBuilder<'a> {
    pub(crate) raw_tables: Vec<Arc<BasiliqDbScannerTable>>,
    pub(crate) tables: BTreeMap<String, BasiliqStoreTableBuilder<'a>>,
    pub(crate) relationships:
        BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
}

#[derive(Debug, Clone)]
pub struct BasiliqStoreTableBuilder<'a> {
    pub(crate) table: Arc<postgres_metadata::parsed::BasiliqDbScannerTable>,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) properties: MessyJsonObject<'a>,
}

impl<'a> BasiliqStoreBuilder<'a> {
    pub fn check_schema(table: &BasiliqDbScannerTable) -> Option<&BasiliqDbScannerTable> {
        match POSTGRES_SYSTEM_SCHEMA.contains(&table.schema().name().as_str())
		// If in a system schema
		{
			true => None,
			false => Some(table)
		}
    }

    pub fn new(raw_tables: Vec<Arc<BasiliqDbScannerTable>>) -> Self {
        let mut res: BTreeMap<String, BasiliqStoreTableBuilder<'_>> = BTreeMap::new();
        let mut relationships: BTreeMap<String, BTreeMap<String, (String, i16)>> = BTreeMap::new();
        for table in raw_tables.iter() {
            if let Some((table_builder, fkey)) = Self::check_schema(&table)
                .map(|table| (table, Self::build_fkeys_raw(&table)))
                .and_then(|(table, fkeys)| Self::build_pkeys(&table).map(|x| (x, fkeys)))
                .and_then(|(pkey, fkey)| {
                    Self::build_object(table.clone(), pkey, &fkey).map(|x| (x, fkey))
                })
            {
                let nfkey = extract_relationships_fields_name(fkey, &table_builder);
                let name = name::create_resource_name(&table);
                res.insert(name.clone(), table_builder);
                relationships.insert(name, nfkey);
            }
        }
        let relationships = Self::build_relationships_base(&res, relationships);
        BasiliqStoreBuilder {
            raw_tables,
            tables: res,
            relationships,
        }
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
