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
    /// Take a first stab at parsing the relationships
    /// Every relationships will first be expressed as a OneToMany and ManyToOne relationships
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
            // Iterate over every tables containing relationships
            if !tables.contains_key(main_table_name) {
                warn!("Unknown table `{}`, skipping...", main_table_name);
                continue 'tables;
            }
            'rels: for (rel_key, (rel_type, rel_field_index)) in rels {
                // Iterate over every relationships in that table
                let ftable = match tables.get(rel_type) {
                    // Get the type of the foreign table, fails if can't be found
                    Some(x) => x,
                    None => {
                        warn!("Unknown table `{}`, skipping...", rel_type);
                        continue 'tables;
                    }
                };
                let fkey_col_name = match ftable // Get the column name of the field that is referenced, fails if can't be found
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
                    // Insert the ManyToOne relationship from the main type to the foreign type
                    gen_rels_identifier(&res, main_table_name.clone(), rel_key.clone()),
                    BasiliqStoreRelationshipData {
                        ftable_name: rel_type.clone(),
                        ffield_name: fkey_col_name.clone(),
                        type_: BasiliqStoreRelationshipType::ManyToOne,
                    },
                );
                res.insert(
                    // Insert the OneToMany relationship from the foreign type to the main type
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

    /// Pull together relationships that can be expressed as ManyToMany through another intermediate table
    pub(super) fn build_relationships_many(
        mut relationships: BTreeMap<
            BasiliqStoreRelationshipIdentifier,
            BasiliqStoreRelationshipData,
        >,
    ) -> BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData> {
        // A store that'll be used for building. It'll map each table with every incoming relationships
        let mut set: BTreeMap<BasiliqStoreTableIdentifier, Vec<BasiliqStoreRelationshipData>> =
            BTreeMap::new();

        // Fill the set
        fill_relationships_set(&mut set, &relationships);
        for (ident, elements) in set.into_iter() {
            // For every table that has relationships
            for element in elements.iter() {
                for other_element in elements.iter() {
                    // Map together the relationships that a common node
                    if other_element == element {
                        // Skip if the same type
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

// It'll map each table with every incoming relationships
fn fill_relationships_set(
    set: &mut BTreeMap<BasiliqStoreTableIdentifier, Vec<BasiliqStoreRelationshipData>>,
    relationships: &BTreeMap<BasiliqStoreRelationshipIdentifier, BasiliqStoreRelationshipData>,
) {
    for (ident, rel_data) in relationships.iter() {
        if let BasiliqStoreRelationshipType::ManyToOne = rel_data.type_() {
            if let Some(x) = set.get_mut(&ident.table_id()) {
                x.push(rel_data.clone());
            } else {
                set.insert(ident.table_id().clone(), vec![rel_data.clone()]);
            }
        }
    }
}
