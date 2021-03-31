use log::warn;

use super::*;

fn gen_rels_identifier(
    map: &BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
    table_name: String,
    field_name: String,
) -> BasiliqStoreRelationshipIdentifier {
    let mut identifier = BasiliqStoreRelationshipIdentifier {
        table_name,
        field_name,
        index: 0,
    };
    while map.contains_key(&identifier) {
        identifier.index += 1;
    }
    identifier
}

impl<'a> BasiliqStoreBuilder<'a> {
    pub(super) fn build_relationships_base(
        tables: &BTreeMap<String, BasiliqStoreTableBuilder<'_>>,
        relationships: BTreeMap<String, BTreeMap<String, (String, i16)>>,
    ) -> BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData> {
        let mut res: BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData> =
            BTreeMap::new();
        'tables: for (main_table_name, rels) in relationships.iter() {
            if !tables.contains_key(main_table_name) {
                warn!("Unknown table `{}`, skipping...", main_table_name);
                continue 'tables;
            }
            'rels: for (rel_key, (rel_type, rel_field_index)) in rels {
                let ftable = match tables.get(rel_type) {
                    Some(x) => x,
                    None => {
                        warn!("Unknown table `{}`, skipping...", rel_type);
                        continue 'tables;
                    }
                };
                let fkey_col_name = match ftable
                    .table
                    .columns_by_id()
                    .get(rel_field_index)
                    .map(|c| c.column().name().to_string())
                {
                    Some(x) => x,
                    None => {
                        warn!(
                            "Unknown column {} for table `{}`, skipping...",
                            rel_field_index, rel_type
                        );
                        continue 'rels;
                    }
                };
                res.insert(
                    gen_rels_identifier(&res, main_table_name.clone(), rel_key.clone()),
                    BasiliqStoreRelationshipData {
                        ftable_name: rel_type.clone(),
                        ffield_name: fkey_col_name.clone(),
                        type_: BasiliqStoreRelationshipType::ManyToOne,
                    },
                );
                res.insert(
                    gen_rels_identifier(&res, rel_type.clone(), fkey_col_name.clone()),
                    BasiliqStoreRelationshipData {
                        ftable_name: main_table_name.clone(),
                        ffield_name: rel_key.clone(),
                        type_: BasiliqStoreRelationshipType::OneToMany,
                    },
                );
            }
        }
        res
    }
}
