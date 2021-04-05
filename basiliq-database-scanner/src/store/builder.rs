use super::*;
use ciboulette::{
    CibouletteRelationshipManyToManyOptionBuilder, CibouletteRelationshipOneToManyOptionBuilder,
    CibouletteStoreBuilder,
};
use ciboulette2postgres::{
    Ciboulette2PostgresId, Ciboulette2PostgresSafeIdent, Ciboulette2PostgresTable,
    Ciboulette2PostgresTableStore, Ciboulette2SqlError,
};
use std::convert::TryFrom;
#[derive(Debug, Clone, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct BasiliqStoreBuilder<'a> {
    pub(crate) raw_tables: Vec<Arc<BasiliqDbScannedTable>>,
    pub(crate) tables: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable<'a>>,
    pub(crate) aliases: BiBTreeMap<BasiliqStoreTableIdentifier, String>,
    pub(crate) config: BasiliqStoreConfig,
}

impl<'a> BasiliqStoreBuilder<'a> {
    pub fn build(self) -> Result<BasiliqStore<'a>, Ciboulette2SqlError> {
        let mut ciboulette_store_builder = CibouletteStoreBuilder::default();
        let mut ciboulette_table_store = Ciboulette2PostgresTableStore::default();

        for (table, alias) in self.tables().values().zip(self.aliases().right_values()) {
            ciboulette_store_builder.add_type(
                alias.as_str(),
                *table.id_type(),
                table.properties().clone(),
            )?;
        }
        for (table, alias) in self.tables().values().zip(self.aliases().right_values()) {
            for (k, v) in table.relationships() {
                match v.type_() {
                    BasiliqStoreRelationshipType::OneToMany(x) if !x => {
                        let one_type = ciboulette_store_builder.get_type(alias)?.clone();
                        let many_type = ciboulette_store_builder
                            .get_type(self.aliases().get_by_left(v.ftable_name()).ok_or_else(
                                || Ciboulette2SqlError::UnknownTable(v.ftable_name().to_string()),
                            )?)?
                            .clone();
                        ciboulette_store_builder.add_one_to_many_rel_no_reverse(
                            CibouletteRelationshipOneToManyOptionBuilder::new(
                                one_type,
                                many_type,
                                v.ffield_name().clone(),
                                false, //FIXME
                            ),
                            Some(k),
                        )?;
                    }
                    BasiliqStoreRelationshipType::ManyToOne(x) if !x => {
                        let many_type = ciboulette_store_builder.get_type(alias)?.clone();
                        let one_type = ciboulette_store_builder
                            .get_type(self.aliases().get_by_left(v.ftable_name()).ok_or_else(
                                || Ciboulette2SqlError::UnknownTable(v.ftable_name().to_string()),
                            )?)?
                            .clone();
                        ciboulette_store_builder.add_one_to_many_rel_no_reverse(
                            CibouletteRelationshipOneToManyOptionBuilder::new(
                                one_type,
                                many_type,
                                v.ffield_name().clone(),
                                false, // FIXME
                            ),
                            Some(alias.as_str()),
                        )?;
                    }
                    BasiliqStoreRelationshipType::ManyToMany(opt) => {
                        let bucket_alias =
                            self.aliases().get_by_left(opt.bucket()).ok_or_else(|| {
                                Ciboulette2SqlError::UnknownTable(v.ftable_name().to_string())
                            })?;
                        let bucket_type = ciboulette_store_builder.get_type(bucket_alias)?.clone();
                        let ltype_alias =
                            self.aliases().get_by_left(v.ltable_name()).ok_or_else(|| {
                                Ciboulette2SqlError::UnknownTable(v.ftable_name().to_string())
                            })?;
                        let rtype_alias =
                            self.aliases().get_by_left(v.ftable_name()).ok_or_else(|| {
                                Ciboulette2SqlError::UnknownTable(v.ftable_name().to_string())
                            })?;
                        let ltable_type = ciboulette_store_builder.get_type(ltype_alias)?.clone();
                        let rtable_type = ciboulette_store_builder.get_type(rtype_alias)?.clone();
                        ciboulette_store_builder.add_many_to_many_rel_no_reverse(
                            ltype_alias,
                            (rtype_alias, Some(k)),
                            CibouletteRelationshipManyToManyOptionBuilder::new(
                                bucket_type,
                                [
                                    (ltable_type, opt.lfield_name().clone()),
                                    (rtable_type, opt.ffield_name().clone()),
                                ],
                            ),
                        )?;
                    }
                    _ => continue,
                }
            }
        }
        let ciboulette_store = ciboulette_store_builder.build()?;
        for (table, alias) in self.tables().values().zip(self.aliases().right_values()) {
            ciboulette_table_store.add_table(
                alias.clone(),
                Arc::new(Ciboulette2PostgresTable::new(
                    Ciboulette2PostgresId::new_from_ciboulette_id_type(
                        *table.id_type(),
                        table.id_name().as_str(),
                    )?,
                    Some(Ciboulette2PostgresSafeIdent::try_from(
                        table.table().schema().name().clone(),
                    )?),
                    Ciboulette2PostgresSafeIdent::try_from(table.table().table().name().clone())?,
                    ciboulette_store.get_type(alias.as_str())?.clone(),
                )),
            )
        }
        Ok(BasiliqStore {
            ciboulette: ciboulette_store,
            tables: ciboulette_table_store,
            config: self.config,
        })
    }
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreTableBuilder<'a> {
    pub(crate) table: Arc<postgres_metadata::parsed::BasiliqDbScannedTable>,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) id_name: String,
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
            id_name: self.id_name,
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
    pub(crate) id_name: String,
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
        let mut builder = BasiliqStoreBuilder {
            raw_tables,
            tables: table_store,
            aliases,
            config: BasiliqStoreConfig::default(),
        };
        builder.config = builder.gen_config();
        builder
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
