use super::*;
use itertools::{EitherOrBoth, Itertools};

#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct BasiliqStoreBuilder<'a> {
    pub(crate) raw_tables: Vec<Arc<BasiliqDbScannedTable>>,
    pub(crate) tables: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable<'a>>,
    pub(crate) aliases: BiBTreeMap<BasiliqStoreTableIdentifier, String>,
}

impl<'a> BasiliqStoreConfigMergeable<BasiliqStoreConfig> for BasiliqStoreBuilder<'a> {
    fn basiliq_config_merge(
        &mut self,
        other: &BasiliqStoreConfig,
    ) -> Result<(), BasiliqStoreConfigError> {
        for (resource_name, resource_cfg) in other.resources() {
            let table_ident = BasiliqStoreTableIdentifier::from(resource_cfg);
            if !resource_cfg.enabled
            // Disable that resource
            {
                self.tables_mut().remove(&table_ident);
                self.aliases_mut().remove_by_left(&table_ident);
                continue;
            }
            self.aliases_mut()
                .insert(table_ident.clone(), resource_name.clone());
            if let Some(table) = self.tables().get(&table_ident) {
                let mut new_rel: BTreeMap<String, BasiliqStoreRelationshipData> =
                    table.relationships().clone();

                for x in table.relationships().iter().merge_join_by(
                    resource_cfg.relationships().iter(),
                    |(_k1, v1), (_k2, v2)| v1.ftable_name().cmp(v2.target()),
                ) {
                    match x {
                        EitherOrBoth::Both((k1, _v1), (k2, _v2)) => {
                            new_rel
                                .remove(k1)
                                .and_then(|x| new_rel.insert(k2.clone(), x));
                        }
                        EitherOrBoth::Left((k1, _v1)) => {
                            new_rel.remove(k1);
                        }
                        EitherOrBoth::Right((_, _)) => {
                            warn!("Can't insert new relationships from the configuration. Skipping...");
                        }
                    };
                }
                let new_rel: BTreeMap<String, BasiliqStoreRelationshipData> = new_rel
                    .into_iter()
                    .filter(|(_, v)| self.tables().contains_key(v.ftable_name()))
                    .collect();
                if let Some(table) = self.tables_mut().get_mut(&table_ident) {
                    table.relationships = new_rel
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
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

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
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

    pub fn get_table(&self, ident: &BasiliqStoreTableIdentifier) -> Option<&'a BasiliqStoreTable> {
        self.tables().get(ident)
    }

    pub fn get_table_by_alias(&self, alias: &str) -> Option<&'a BasiliqStoreTable> {
        self.aliases()
            .get_by_right(alias)
            .and_then(|ident| self.tables().get(ident))
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

    /// Extract data from the raw table, to build the builder
    fn extract_data_from_raw_tables(
        raw_tables: Vec<Arc<BasiliqDbScannedTable>>,
    ) -> (
        BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder<'a>>,
        BTreeSet<BasiliqStoreRelationshipData>,
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
    ) -> BiBTreeMap<BasiliqStoreTableIdentifier, String> {
        let aliases = table_builder_store
            .iter()
            .map(|(x, _)| (x.clone(), format!("{}__{}", x.schema_name, x.table_name)))
            .collect();
        aliases
    }

    /// Build the relationships
    fn build_relationships(
        relationships: BTreeSet<BasiliqStoreRelationshipData>,
        table_builder_store: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder<'a>>,
    ) -> BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable<'a>> {
        let mut table_store: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable> =
            table_builder_store
                .into_iter()
                .map(|(k, v)| (k, v.build(std::iter::empty())))
                .collect();

        for rel_data in relationships.into_iter() {
            if let Some(table) = table_store.get_mut(rel_data.ltable_name()) {
                inserts_relationship(table, rel_data);
            }
        }
        table_store
    }
}

/// Insert a new relationships into the provided table
fn inserts_relationship(table: &mut BasiliqStoreTable, rel_data: BasiliqStoreRelationshipData) {
    match table.relationships.get(rel_data.ftable_name().table_name()) {
        Some(x) if x == &rel_data => (),
        Some(_) => {
            let mut counter: usize = 0;
            let mut name = format!("{}_{}", rel_data.ftable_name().table_name(), counter);
            while table.relationships.contains_key(&name) {
                counter += 1;
                name = format!("{}_{}", rel_data.ftable_name().table_name(), counter);
            }
            table.relationships.insert(name, rel_data);
        }
        None => {
            table
                .relationships
                .insert(rel_data.ftable_name().table_name().clone(), rel_data);
        }
    }
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
