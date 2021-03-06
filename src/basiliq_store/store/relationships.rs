use super::*;

impl BasiliqStoreBuilder {
    /// Take a first stab at parsing the relationships
    /// Every relationships will first be expressed as a OneToMany and ManyToOne relationships
    pub(super) fn build_relationships_base(
        tables: &BTreeMap<BasiliqStoreTableIdentifier, BasiliqStoreTableBuilder>,
        relationships: BTreeMap<
            BasiliqStoreTableIdentifier,
            BTreeMap<String, (BasiliqStoreTableIdentifier, i16)>,
        >,
    ) -> BTreeSet<BasiliqStoreRelationshipData> {
        let mut res: BTreeSet<BasiliqStoreRelationshipData> = BTreeSet::new();
        'tables: for (main_table_name, rels) in relationships.iter() {
            // Iterate over every tables containing relationships
            if !tables.contains_key(main_table_name) {
                warn!("Unknown table `{}`, skipping...", main_table_name);
                continue 'tables;
            }
            let ltable = match tables.get(main_table_name) {
                // Get the type of the foreign table, fails if can't be found
                Some(x) => x,
                None => {
                    warn!("Unknown table `{}`, skipping...", main_table_name);
                    continue 'tables;
                }
            };
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
                let lkey_col = match ltable // Get the column name of the field that is referenced, fails if can't be found
                    .table
                    .columns_by_name()
                    .get(rel_key)
                {
                    Some(x) => x,
                    None => {
                        warn!(
                            "Unknown column {} for table `{}`, skipping...",
                            main_table_name, main_table_name
                        );
                        continue 'rels;
                    }
                };
                let fkey_col = match ftable // Get the column name of the field that is referenced, fails if can't be found
                    .table
                    .columns_by_id()
                    .get(rel_field_index)
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
                    BasiliqStoreRelationshipData {
                        ltable: main_table_name.clone(),
                        ftable: rel_type.clone(),
                        ffield_name: ArcStr::from(fkey_col.column().name()),
                        lfield_name: ArcStr::from(rel_key),
                        type_: BasiliqStoreRelationshipType::ManyToOne(false),
                        optional: !lkey_col.column().non_null(),
                    },
                );
                if ltable != ftable
                // No need to insert both if it's a self reference
                {
                    res.insert(
                        // Insert the OneToMany relationship from the foreign type to the main type
                        BasiliqStoreRelationshipData {
                            ltable: rel_type.clone(),
                            ftable: main_table_name.clone(),
                            ffield_name: ArcStr::from(rel_key),
                            lfield_name: ArcStr::from(fkey_col.column().name()),
                            type_: BasiliqStoreRelationshipType::OneToMany(false),
                            optional: !lkey_col.column().non_null(),
                        },
                    );
                }
            }
        }
        res
    }

    /// Pull together relationships that can be expressed as ManyToMany through another intermediate table
    pub(super) fn build_relationships_many(
        mut relationships: BTreeSet<BasiliqStoreRelationshipData>,
    ) -> BTreeSet<BasiliqStoreRelationshipData> {
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
                    let mut old_rel_1 = element.clone();
                    let mut old_rel_2 = other_element.clone();
                    relationships.remove(&old_rel_1);
                    relationships.remove(&old_rel_2);
                    old_rel_1.type_ = BasiliqStoreRelationshipType::OneToMany(true);
                    old_rel_2.type_ = BasiliqStoreRelationshipType::OneToMany(true);
                    relationships.insert(old_rel_1);
                    relationships.insert(old_rel_2);
                    relationships.insert(BasiliqStoreRelationshipData {
                        ltable: element.ltable().clone(),
                        lfield_name: element.lfield_name().clone(),
                        ftable: other_element.ltable().clone(),
                        ffield_name: other_element.lfield_name().clone(),
                        type_: BasiliqStoreRelationshipType::ManyToMany(
                            BasiliqStoreRelationshipManyToManyData {
                                bucket: ident.clone(),
                                lfield_name: element.ffield_name().clone(),
                                ffield_name: other_element.ffield_name().clone(),
                            },
                        ),
                        optional: true,
                    });
                }
            }
        }
        relationships
    }
}

// It'll map each table with every incoming relationships
fn fill_relationships_set(
    set: &mut BTreeMap<BasiliqStoreTableIdentifier, Vec<BasiliqStoreRelationshipData>>,
    relationships: &BTreeSet<BasiliqStoreRelationshipData>,
) {
    for rel_data in relationships.iter() {
        if let BasiliqStoreRelationshipType::OneToMany(_) = rel_data.type_() {
            if let Some(x) = set.get_mut(&rel_data.ftable()) {
                x.push(rel_data.clone());
            } else {
                set.insert(rel_data.ftable().clone(), vec![rel_data.clone()]);
            }
        }
    }
}
