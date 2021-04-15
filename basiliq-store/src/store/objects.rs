use super::*;

impl BasiliqStoreBuilder {
    fn type_to_messy_json(
        col_settings: &BasiliqDbScannerColumn,
        type_: &BasiliqDbScannerTypeRaw,
    ) -> Option<MessyJson> {
        let required: bool = col_settings.column().non_null();

        match type_.category() {
            BasiliqDbScannerTypeCategory::Array => {
                trace!(
                    ">> Found an array for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::Composite => {
                trace!(
                    ">> Found a composite for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::Enum => Some(MessyJson::from(MessyJsonInner::String(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Geo => Some(MessyJson::from(MessyJsonInner::String(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::NetworkAddress => Some(MessyJson::from(
                MessyJsonInner::String(MessyJsonScalar::new(!required)),
            )),
            BasiliqDbScannerTypeCategory::Pseudo => Some(MessyJson::from(MessyJsonInner::String(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Range => Some(MessyJson::from(MessyJsonInner::String(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::DateTime => Some(MessyJson::from(
                MessyJsonInner::String(MessyJsonScalar::new(!required)),
            )),
            BasiliqDbScannerTypeCategory::Numeric => Some(MessyJson::from(MessyJsonInner::Number(
                MessyJsonNumeric::new(MessyJsonNumberType::U64, !required),
            ))),
            BasiliqDbScannerTypeCategory::String => Some(MessyJson::from(MessyJsonInner::String(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Timespan => Some(MessyJson::from(
                MessyJsonInner::String(MessyJsonScalar::new(!required)),
            )),
            BasiliqDbScannerTypeCategory::UserDefined => match type_.name().as_str() {
                "uuid" => Some(MessyJson::from(MessyJsonInner::Uuid(MessyJsonScalar::new(
                    !required,
                )))),
                _ => {
                    trace!(
                        ">> Found an user-defined for type {}. Unsupported, skipping..",
                        type_.name()
                    );
                    None
                }
            },
            BasiliqDbScannerTypeCategory::Unknown => {
                trace!(
                    ">> Found an unknown for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::BitString => Some(MessyJson::from(
                MessyJsonInner::String(MessyJsonScalar::new(!required)),
            )),
            BasiliqDbScannerTypeCategory::Boolean => Some(MessyJson::from(MessyJsonInner::Bool(
                MessyJsonScalar::new(!required),
            ))),
        }
    }

    fn type_to_id(col_settings: &BasiliqDbScannerColumn) -> Option<CibouletteIdType> {
        match col_settings.type_() {
            BasiliqDbScannedType::Simple(type_) => match type_.category() {
                BasiliqDbScannerTypeCategory::Array => {
                    trace!(
                        ">> Found an array for type {}. Unsupported for an ID, skipping..",
                        type_.name()
                    );
                    None
                }
                BasiliqDbScannerTypeCategory::Composite => {
                    trace!(
                        ">> Found a composite for type {}. Unsupported for an ID, skipping..",
                        type_.name()
                    );
                    None
                }
                BasiliqDbScannerTypeCategory::Enum => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::Geo => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::NetworkAddress => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::Pseudo => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::Range => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::DateTime => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::Numeric => Some(CibouletteIdType::Number),
                BasiliqDbScannerTypeCategory::String => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::Timespan => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::UserDefined => match type_.name().as_str() {
                    "uuid" => Some(CibouletteIdType::Uuid),
                    _ => {
                        trace!(
								">> Found an user-defined for type {}. Unsupported for an ID, skipping..",
								type_.name()
							);
                        None
                    }
                },
                BasiliqDbScannerTypeCategory::Unknown => {
                    trace!(
                        ">> Found an unknown for type {}. Unsupported for an ID, skipping..",
                        type_.name()
                    );
                    None
                }
                BasiliqDbScannerTypeCategory::BitString => Some(CibouletteIdType::Text),
                BasiliqDbScannerTypeCategory::Boolean => {
                    trace!(
                        ">> Found an unknown for type {}. Unsupported for an ID, skipping..",
                        type_.name()
                    );
                    None
                }
            },
            BasiliqDbScannedType::Nested(_, _) => {
                trace!("Found an array. Unsupported for an id, skipping..");
                None
            }
        }
    }

    pub fn build_object(
        table: Arc<BasiliqDbScannedTable>,
        pkey: i16,
        fkeys: &BTreeMap<i16, (BasiliqStoreTableIdentifier, i16)>,
    ) -> Option<BasiliqStoreTableBuilder> {
        let mut obj_properties: BTreeMap<String, MessyJson> = BTreeMap::new();
        let mut pkey_type: Option<(CibouletteIdType, String)> = None;
        trace!(
            "Scanning table {} in schema {}",
            table.table().name(),
            table.schema().name()
        );

        for (id, col_settings) in table.columns_by_id() {
            if pkey == *id {
                // If the primary key
                pkey_type = Self::type_to_id(col_settings)
                    .map(|x| (x, col_settings.column().name().clone()));
                continue;
            }
            if fkeys.contains_key(id) {
                // If a foreign key
                continue;
            }
            if POSTGRES_SYSTEM_COLUMNS.contains(&col_settings.column().name().as_str()) {
                // If a system column
                continue;
            }
            trace!("> Scanning columns {}", col_settings.column().name());
            if let Some(obj) = match col_settings.type_() {
                BasiliqDbScannedType::Simple(type_) => {
                    Self::type_to_messy_json(col_settings, type_)
                }
                BasiliqDbScannedType::Nested(_parent, _child) => {
                    trace!("Found an array. Unsupported, skipping..");
                    None
                }
            } {
                obj_properties.insert(col_settings.column().name().clone(), obj);
            }
        }
        pkey_type
            .map(|x| {
                (
                    x,
                    MessyJsonObject::from(MessyJsonObjectInner::new(
                        obj_properties
                            .into_iter()
                            .map(|(k, v)| (ArcStr::from(k), v))
                            .collect(),
                        false,
                    )),
                )
            })
            .map(|(id, properties)| BasiliqStoreTableBuilder {
                table,
                properties,
                id_type: id.0,
                id_name: id.1,
            })
    }
}
