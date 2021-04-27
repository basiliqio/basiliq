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
pub struct BasiliqStoreBuilder {
    pub(crate) tables: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable>,
    pub(crate) aliases: BiBTreeMap<BasiliqStoreTableIdentifier, String>,
    pub(crate) config: BasiliqStoreConfig,
}

impl BasiliqStoreBuilder {
    pub fn build(self) -> Result<BasiliqStore, Ciboulette2SqlError> {
        let mut ciboulette_store_builder = CibouletteStoreBuilder::default();
        let mut ciboulette_table_store = Ciboulette2PostgresTableStore::default();
        let mut already_built_rel: BTreeSet<(ArcStr, ArcStr, ArcStr)> = BTreeSet::new();

        for (table, alias) in self.tables().values().zip(self.aliases().right_values()) {
            ciboulette_store_builder.add_type(
                alias.as_str(),
                *table.id_type(),
                table.properties().clone(),
            )?;
        }
        self.insert_one_to_many(&mut ciboulette_store_builder, &mut already_built_rel)?;
        self.insert_many_to_many(&mut ciboulette_store_builder)?;
        let ciboulette_store = ciboulette_store_builder.build()?;
        for (table, alias) in self.tables().values().zip(self.aliases().right_values()) {
            ciboulette_table_store.add_table(
                alias.clone(),
                Arc::new(Ciboulette2PostgresTable::new(
                    Ciboulette2PostgresId::new_from_ciboulette_id_type(
                        *table.id_type(),
                        table.id_name(),
                    )?,
                    Some(Ciboulette2PostgresSafeIdent::try_from(ArcStr::from(
                        table.table().schema().name(),
                    ))?),
                    Ciboulette2PostgresSafeIdent::try_from(ArcStr::from(
                        table.table().table().name(),
                    ))?,
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

    fn insert_many_to_many(
        &self,
        ciboulette_store_builder: &mut CibouletteStoreBuilder,
    ) -> Result<(), Ciboulette2SqlError> {
        for table in self.tables().values() {
            for (rel_alias, rel_data) in table.relationships() {
                match rel_data.type_() {
                    BasiliqStoreRelationshipType::ManyToMany(rel_opt) => {
                        self.inserts_many_to_many_routine(
                            // Insert many to many rels
                            ciboulette_store_builder,
                            rel_opt,
                            rel_data,
                            rel_alias,
                        )?;
                    }
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    fn insert_one_to_many(
        &self,
        ciboulette_store_builder: &mut CibouletteStoreBuilder,
        already_built_rel: &mut BTreeSet<(ArcStr, ArcStr, ArcStr)>,
    ) -> Result<(), Ciboulette2SqlError> {
        for (table, alias) in self.tables().values().zip(self.aliases().right_values()) {
            for (rel_alias, v) in table.relationships() {
                let (one_type, alias_one, many_type, alias_many, many_table_key) = match v.type_() {
                    BasiliqStoreRelationshipType::OneToMany(_) => {
                        let many_alias =
                            self.aliases().get_by_left(v.ftable()).ok_or_else(|| {
                                Ciboulette2SqlError::UnknownTable(v.ftable().to_string())
                            })?;
                        let many_type = ciboulette_store_builder.get_type(&many_alias)?.clone();
                        let one_type = ciboulette_store_builder.get_type(alias)?.clone();
                        let one_name = one_type.name().clone();
                        (
                            one_type,
                            one_name,
                            many_type,
                            rel_alias.clone(),
                            v.ffield_name().clone(),
                        )
                    }
                    BasiliqStoreRelationshipType::ManyToOne(_) => {
                        let one_alias =
                            self.aliases().get_by_left(v.ftable()).ok_or_else(|| {
                                Ciboulette2SqlError::UnknownTable(v.ftable().to_string())
                            })?;
                        let one_type = ciboulette_store_builder.get_type(&one_alias)?.clone();
                        let many_type = ciboulette_store_builder.get_type(alias)?.clone();
                        let many_name = many_type.name().clone();
                        (
                            one_type,
                            rel_alias.clone(),
                            many_type,
                            many_name,
                            v.lfield_name().clone(),
                        )
                    }
                    BasiliqStoreRelationshipType::ManyToMany(_) => continue,
                };
                if already_built_rel.contains(&(
                    one_type.name().clone(),
                    many_table_key.clone(),
                    many_type.name().clone(),
                )) {
                    // Already exists
                    continue;
                }
                already_built_rel.insert((
                    one_type.name().clone(),
                    many_table_key.clone(),
                    many_type.name().clone(),
                ));
                ciboulette_store_builder.add_one_to_many_rel(
                    CibouletteRelationshipOneToManyOptionBuilder::new(
                        one_type,
                        many_type,
                        many_table_key,
                        v.optional(),
                    ),
                    Some(alias_one),
                    Some(alias_many),
                )?;
            }
        }
        Ok(())
    }

    fn inserts_many_to_many_routine(
        &self,
        ciboulette_store_builder: &mut CibouletteStoreBuilder,
        rel_opt: &BasiliqStoreRelationshipManyToManyData,
        rel_data: &BasiliqStoreRelationshipData,
        rel_alias: &ArcStr,
    ) -> Result<(), Ciboulette2SqlError> {
        let bucket_alias = self
            .aliases()
            .get_by_left(rel_opt.bucket())
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(rel_data.ftable().to_string()))?;
        let bucket_type = ciboulette_store_builder.get_type(bucket_alias)?.clone();
        let ltype_alias = self
            .aliases()
            .get_by_left(rel_data.ltable())
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(rel_data.ftable().to_string()))?;
        let rtype_alias = self
            .aliases()
            .get_by_left(rel_data.ftable())
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(rel_data.ftable().to_string()))?;
        let ltable_type = ciboulette_store_builder.get_type(ltype_alias)?.clone();
        let rtable_type = ciboulette_store_builder.get_type(rtype_alias)?.clone();
        ciboulette_store_builder.add_many_to_many_rel_no_reverse_direct_only(
            ltype_alias,
            (rtype_alias, Some(rel_alias)),
            CibouletteRelationshipManyToManyOptionBuilder::new(
                bucket_type,
                [
                    (ltable_type, rel_opt.lfield_name().clone()),
                    (rtable_type, rel_opt.ffield_name().clone()),
                ],
            ),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqStoreTableBuilder {
    pub(crate) table: Arc<postgres_metadata::parsed::BasiliqDbScannedTable>,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) id_name: String,
    pub(crate) properties: MessyJsonObject,
}

impl BasiliqStoreTableBuilder {
    pub fn build<I>(self, relationships: I) -> BasiliqStoreTable
    where
        I: IntoIterator<Item = (String, BasiliqStoreRelationshipData)>,
    {
        BasiliqStoreTable {
            table: self.table,
            id_type: self.id_type,
            id_name: ArcStr::from(self.id_name),
            properties: self.properties,
            relationships: relationships
                .into_iter()
                .map(|(k, v)| (ArcStr::from(k), v))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct BasiliqStoreTable {
    pub(crate) table: Arc<postgres_metadata::parsed::BasiliqDbScannedTable>,
    pub(crate) id_type: CibouletteIdType,
    pub(crate) id_name: ArcStr,
    pub(crate) relationships: BTreeMap<ArcStr, BasiliqStoreRelationshipData>,
    pub(crate) properties: MessyJsonObject,
}

impl BasiliqStoreBuilder {
    pub fn check_schema(table: &BasiliqDbScannedTable) -> Option<&BasiliqDbScannedTable> {
        match POSTGRES_SYSTEM_SCHEMA.contains(&table.schema().name().as_str())
		// If in a system schema
		{
			true => None,
			false => Some(table)
		}
    }

    pub fn get_table(&self, ident: &BasiliqStoreTableIdentifier) -> Option<&BasiliqStoreTable> {
        self.tables().get(ident)
    }

    pub fn get_table_by_alias(&self, alias: &str) -> Option<&BasiliqStoreTable> {
        self.aliases()
            .get_by_right(alias)
            .and_then(|ident| self.tables().get(ident))
    }

    /// Create a new builder
    pub fn new(raw_tables: Vec<Arc<BasiliqDbScannedTable>>) -> Self {
        let (table_builder_store, relationships) = Self::extract_data_from_raw_tables(raw_tables);
        let aliases = Self::build_alias_map(&table_builder_store);
        let table_store = Self::build_relationships(relationships, table_builder_store);
        let mut builder = BasiliqStoreBuilder {
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
        BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder>,
        BTreeSet<BasiliqStoreRelationshipData>,
    ) {
        let mut table_builder_store: BTreeMap<
            BasiliqStoreTableIdentifier,
            BasiliqStoreTableBuilder,
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
            .map(|(x, _)| (x.clone(), format!("{}__{}", x.schema, x.table)))
            .collect();
        aliases
    }

    /// Build the relationships
    fn build_relationships(
        relationships: BTreeSet<BasiliqStoreRelationshipData>,
        table_builder_store: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder>,
    ) -> BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable> {
        let mut table_store: BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTable> =
            table_builder_store
                .into_iter()
                .map(|(k, v)| (k, v.build(std::iter::empty())))
                .collect();

        for rel_data in relationships.into_iter() {
            if let Some(table) = table_store.get_mut(rel_data.ltable()) {
                inserts_relationship(table, rel_data);
            }
        }
        table_store
    }
}

/// Insert a new relationships into the provided table
fn inserts_relationship(table: &mut BasiliqStoreTable, rel_data: BasiliqStoreRelationshipData) {
    let base_name = format!(
        "{}__{}",
        rel_data.ftable().schema(),
        rel_data.ftable().table()
    );
    match table.relationships.get(base_name.as_str()) {
        Some(x) if x == &rel_data => (),
        Some(_) => {
            let mut counter: usize = 0;
            let mut name = format!("{}_{}", base_name, counter);
            while table.relationships.contains_key(name.as_str()) {
                counter += 1;
                name = format!("{}_{}", base_name, counter);
            }
            table.relationships.insert(ArcStr::from(name), rel_data);
        }
        None => {
            table
                .relationships
                .insert(ArcStr::from(base_name), rel_data);
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
