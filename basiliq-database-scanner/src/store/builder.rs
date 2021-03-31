use super::*;
use ciboulette::CibouletteIdType;
use log::warn;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BasiliqStoreBuilder<'a> {
    pub(crate) raw_tables: Vec<Arc<BasiliqDbScannedTable>>,
    pub(crate) tables: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable<'a>>,
    pub(crate) aliases: BTreeMap<BasiliqStoreTableIdentifier, String>,
}

#[derive(Debug, Clone)]
pub struct BasiliqStoreTableBuilder<'a> {
    pub(crate) table: Arc<postgres_metadata::parsed::BasiliqDbScannedTable>,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) properties: MessyJsonObject<'a>,
}

impl<'a> BasiliqStoreTableBuilder<'a> {
    pub fn build<I>(self, relationships: I) -> BasiliqStoreTable<'a>
    where
        I: IntoIterator<Item = (String, BasiliqStoreRelationshipData)>,
    {
        BasiliqStoreTable {
            table: self.table,
            id_type: self.id_type,
            properties: self.properties,
            relationships: relationships.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasiliqStoreTable<'a> {
    pub(crate) table: Arc<postgres_metadata::parsed::BasiliqDbScannedTable>,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) relationships: BTreeMap<String, BasiliqStoreRelationshipData>,
    pub(crate) properties: MessyJsonObject<'a>,
}

impl<'a> BasiliqStoreBuilder<'a> {
    pub fn check_schema(table: &BasiliqDbScannedTable) -> Option<&BasiliqDbScannedTable> {
        match POSTGRES_SYSTEM_SCHEMA.contains(&table.schema().name().as_str())
		// If in a system schema
		{
			true => None,
			false => Some(table)
		}
    }

    /// Create a new builder
    pub fn new(raw_tables: Vec<Arc<BasiliqDbScannedTable>>) -> Self {
        let (table_builder_store, relationships) =
            Self::extract_data_from_raw_tables(raw_tables.clone());
        let aliases = Self::build_alias_map(&table_builder_store);
        let table_store = Self::build_relationships(relationships, table_builder_store);
        BasiliqStoreBuilder {
            raw_tables,
            tables: table_store,
            aliases,
        }
    }

    // pub fn generate_config(&self) -> BasiliqStoreConfig
    // {
    // 	let mut resource: BTreeMap<String, BasiliqStoreResourceConfig> = BTreeMap::new();

    // 	for (alias, (table_ident, table_builder)) in self.aliases.values().zip(self.tables.iter())
    // 	{
    // 		if resource.contains_key(alias)
    // 		{
    // 			warn!("Duplicate resource name `{}`", alias);
    // 			continue ;
    // 		}
    // 		let relationships
    // 		resource.insert(alias, BasiliqStoreResourceConfig {
    // 			schema: table_ident.schema_name().clone(),
    // 			table_name: table_ident.table_name().clone(),
    // 			enabled: true,

    // 		})
    // 	}
    // 	todo!()
    // }

    /// Extract data from the raw table, to build the builder
    fn extract_data_from_raw_tables(
        raw_tables: Vec<Arc<BasiliqDbScannedTable>>,
    ) -> (
        BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder<'a>>,
        BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
    ) {
        let mut table_builder_store: BTreeMap<
            BasiliqStoreTableIdentifier,
            BasiliqStoreTableBuilder<'_>,
        > = BTreeMap::new();
        let mut relationships: BTreeMap<
            BasiliqStoreTableIdentifier,
            BTreeMap<String, (BasiliqStoreTableIdentifier, i16)>,
        > = BTreeMap::new();
        for table in raw_tables.iter() {
            if let Some((table_builder, fkey)) = Self::check_schema(&table)
                .map(|table| (table, Self::build_fkeys_raw(&table)))
                .and_then(|(table, fkeys)| Self::build_pkeys(&table).map(|x| (x, fkeys)))
                .and_then(|(pkey, fkey)| {
                    Self::build_object(table.clone(), pkey, &fkey).map(|x| (x, fkey))
                })
            {
                let nfkey = extract_relationships_fields_name(fkey, &table_builder);
                table_builder_store
                    .insert(BasiliqStoreTableIdentifier::from(&**table), table_builder);
                relationships.insert(BasiliqStoreTableIdentifier::from(&**table), nfkey);
            }
        }
        let relationships = Self::build_relationships_base(&table_builder_store, relationships);
        let relationships = Self::build_relationships_many(relationships);
        (table_builder_store, relationships)
    }

    /// Build an alias map with default values
    fn build_alias_map(
        table_builder_store: &BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder>,
    ) -> BTreeMap<BasiliqStoreTableIdentifier, String> {
        let aliases = table_builder_store
            .iter()
            .map(|(x, _)| (x.clone(), format!("{}__{}", x.schema_name, x.table_name)))
            .collect();
        aliases
    }

    /// Build the relationships
    fn build_relationships(
        relationships: BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
        mut table_builder_store: BTreeMap<
            BasiliqStoreTableIdentifier,
            BasiliqStoreTableBuilder<'a>,
        >,
    ) -> BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable<'a>> {
        let mut table_store: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable> =
            BTreeMap::new();
        for (ident, rel_data) in relationships.into_iter() {
            match table_store.get_mut(ident.table_id()) {
                Some(table) => {
                    if inserts_relationship(table, &ident, rel_data) {
                        // If the relationships exists, skip
                        continue;
                    }
                }
                None => {
                    if insert_table_and_relationship(
                        // If the table or the relationships exists, skip
                        &mut table_builder_store,
                        ident,
                        &mut table_store,
                        rel_data,
                    ) {
                        continue;
                    }
                }
            }
        }
        table_store
    }
}

/// Insert a new table into the builder store and attached it the provided relationship
fn insert_table_and_relationship<'a>(
    table_builder_store: &mut BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder<'a>>,
    ident: BasiliqStoreRelationshipIdentifier,
    table_store: &mut BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable<'a>>,
    rel_data: BasiliqStoreRelationshipData,
) -> bool {
    {
        let (id, table) = match table_builder_store.remove_entry(ident.table_id()) {
            Some(x) => x,
            None => {
                warn!(
                    "Relationship reference an unknown table `{}`",
                    ident.table_id(),
                );
                return true;
            }
        };
        table_store.insert(
            id,
            table.build(std::iter::once((
                rel_data.ftable_name().table_name().clone(),
                rel_data,
            ))),
        );
    }
    false
}

/// Insert a new relationships into the provided table
fn inserts_relationship(
    table: &mut BasiliqStoreTable,
    ident: &BasiliqStoreRelationshipIdentifier,
    rel_data: BasiliqStoreRelationshipData,
) -> bool {
    match table.relationships.contains_key(ident.field_name()) {
        true => {
            warn!(
                "Duplicate relation on table `{}`, field `{}`",
                ident.table_id(),
                ident.field_name()
            );
            return true;
        }
        false => {
            table
                .relationships
                .insert(rel_data.ftable_name().table_name().clone(), rel_data);
        }
    }
    false
}

/// Extract the field name from a relationships using its column id
fn extract_relationships_fields_name(
    fkey: BTreeMap<i16, (BasiliqStoreTableIdentifier, i16)>,
    table_builder: &BasiliqStoreTableBuilder,
) -> BTreeMap<String, (BasiliqStoreTableIdentifier, i16)> {
    let mut nfkey: BTreeMap<String, (BasiliqStoreTableIdentifier, i16)> = BTreeMap::new();
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
