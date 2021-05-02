use super::*;

impl BasiliqStoreBuilder {
    /// Generate the configuration for the current [BasiliqStoreBuilder](BasiliqStoreBuilder)
    pub(crate) fn gen_config(&self) -> BasiliqStoreConfig {
        let mut resources: BTreeMap<String, BasiliqStoreResourceConfig> = BTreeMap::new();

        for (alias, (table_ident, table_builder)) in
            self.aliases().right_values().zip(self.tables().iter())
        {
            if resources.contains_key(alias) {
                log::warn!("Duplicate resource name `{}`", alias);
                continue;
            }
            let relationships: BTreeMap<ArcStr, BasiliqStoreRelationshipsConfig> = table_builder
                .relationships
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        BasiliqStoreRelationshipsConfig {
                            target: BasiliqStoreTableIdentifier {
                                schema: v.ftable().schema().clone(),
                                table: v.ftable().table().clone(),
                            },
                            through: match v.type_() {
                                BasiliqStoreRelationshipType::ManyToMany(x) => {
                                    Some(BasiliqStoreRelationshipsThroughConfig {
                                        target: x.bucket().clone(),
                                        field: x.ffield_name().clone(),
                                    })
                                }
                                _ => None,
                            },
                            enabled: true,
                            field: v.ffield_name().clone(),
                        },
                    )
                })
                .collect();
            resources.insert(
                alias.clone(),
                BasiliqStoreResourceConfig {
                    target: BasiliqStoreTableIdentifier {
                        schema: table_ident.schema().clone(),
                        table: table_ident.table().clone(),
                    },
                    relationships,
                    enabled: true,
                },
            );
        }
        BasiliqStoreConfig { resources }
    }
}

impl BasiliqStoreConfigMergeable<BasiliqStoreConfig> for BasiliqStoreBuilder {
    fn basiliq_config_merge(
        &mut self,
        other: &BasiliqStoreConfig,
    ) -> Result<(), BasiliqStoreConfigError> {
        for (resource_name, resource_cfg) in other.resources() {
            let table_ident = BasiliqStoreTableIdentifier::from(resource_cfg);
            self.aliases_mut()
                .insert(table_ident.clone(), resource_name.clone());
            match self.tables().get(&table_ident) {
                Some(table) => {
                    let mut new_rel: BTreeMap<ArcStr, BasiliqStoreRelationshipData> =
                        table.relationships().clone();

                    for x in table.relationships().iter().merge_join_by(
                        resource_cfg.relationships().iter(),
                        |(_k1, v1), (_k2, v2)| v1.ftable().cmp(v2.target()),
                    ) {
                        match x {
                            EitherOrBoth::Both((k1, _v1), (k2, _v2)) => {
                                // Rename to the new alias name
                                new_rel
                                    .remove(k1.as_str())
                                    .and_then(|x| new_rel.insert(k2.clone(), x));
                            }
                            EitherOrBoth::Left((_, v1)) => {
                                // It's an error to have a resource we don't know about in the auto-generated configuration
                                return Err(BasiliqStoreConfigError::UnkownResource(
                                    BasiliqStoreConfigErrorSource::BaseConfig,
                                    v1.ltable().clone(),
                                ));
                            }
                            EitherOrBoth::Right((_, v2)) => {
                                // It's an error to have a resource we don't know about in the provided configuration
                                return Err(BasiliqStoreConfigError::UnkownResource(
                                    BasiliqStoreConfigErrorSource::ProvidedConfig,
                                    v2.target().clone(),
                                ));
                            }
                        };
                    }
                    if let Some(table) = self.tables_mut().get_mut(&table_ident) {
                        table.relationships = new_rel
                    }
                }
                None => {
                    return Err(BasiliqStoreConfigError::UnkownResource(
                        BasiliqStoreConfigErrorSource::ProvidedConfig,
                        table_ident.clone(),
                    ))
                }
            }
        }
        self.config = other.clone();
        Ok(())
    }
}
