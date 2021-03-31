use log::warn;

use super::*;

fn gen_rels_identifier(
    map: &BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
    table_id: BasiliqStoreTableIdentifier,
    field_name: String,
) -> BasiliqStoreRelationshipIdentifier {
    let mut identifier = BasiliqStoreRelationshipIdentifier {
        table_id,
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
        tables: &BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder<'_>>,
        relationships: BTreeMap<
            BasiliqStoreTableIdentifier,
            BTreeMap<String, (BasiliqStoreTableIdentifier, i16)>,
        >,
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

    pub(super) fn build_relationships_many(
        mut relationships: BTreeMap<
            BasiliqStoreRelationshipIdentifier,
            BasiliqStoreRelationshipData,
        >,
    ) -> BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData> {
        let mut set: BTreeMap<BasiliqStoreTableIdentifier, Vec<BasiliqStoreRelationshipData>> =
            BTreeMap::new();

        for (ident, rel_data) in relationships.iter() {
            if let BasiliqStoreRelationshipType::ManyToOne = rel_data.type_() {
                if let Some(x) = set.get_mut(&ident.table_id()) {
                    x.push(rel_data.clone());
                } else {
                    set.insert(ident.table_id().clone(), vec![rel_data.clone()]);
                }
            }
        }
        for (ident, elements) in set.into_iter() {
            for element in elements.iter() {
                for other_element in elements.iter() {
                    if other_element == element {
                        continue;
                    }
                    let mut new_ident = BasiliqStoreRelationshipIdentifier::from(element);
                    let new_other_ident = BasiliqStoreRelationshipIdentifier::from(other_element);
                    new_ident.check_index(&relationships);
                    relationships.insert(
                        new_ident,
                        BasiliqStoreRelationshipData {
                            ftable_name: new_other_ident.table_id,
                            ffield_name: ident.to_string(),
                            type_: BasiliqStoreRelationshipType::ManyToMany(ident.clone()),
                        },
                    );
                }
            }
        }
        relationships
    }
}
