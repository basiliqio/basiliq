use super::*;

impl<'a> From<&BasiliqStoreBuilder<'a>> for BasiliqStoreConfig {
    /// Create a config object from a store builder
    fn from(builder: &BasiliqStoreBuilder<'a>) -> Self {
        let mut resources: BTreeMap<String, BasiliqStoreResourceConfig> = BTreeMap::new();

        for (alias, (table_ident, table_builder)) in builder
            .aliases()
            .right_values()
            .zip(builder.tables().iter())
        {
            if resources.contains_key(alias) {
                log::warn!("Duplicate resource name `{}`", alias);
                continue;
            }
            let relationships: BTreeMap<String, BasiliqStoreRelationshipsConfig> = table_builder
                .relationships
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        BasiliqStoreRelationshipsConfig {
                            target: BasiliqStoreTableIdentifier {
                                schema_name: v.ftable_name().schema_name().clone(),
                                table_name: v.ftable_name().table_name().clone(),
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
                        schema_name: table_ident.schema_name().clone(),
                        table_name: table_ident.table_name().clone(),
                    },
                    relationships,
                    enabled: true,
                },
            );
        }
        BasiliqStoreConfig { resources }
    }
}
