use super::*;
use ciboulette::CibouletteIdType;
use log::trace;
use std::sync::Arc;

impl<'a> BasiliqStoreBuilder<'a> {
    fn type_to_messy_json(
        col_settings: &BasiliqDbScannerColumn,
        type_: &BasiliqDbScannerTypeRaw,
    ) -> Option<MessyJson<'a>> {
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
            BasiliqDbScannerTypeCategory::Enum => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Geo => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::NetworkAddress => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Pseudo => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Range => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::DateTime => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Numeric => Some(MessyJson::Number(Cow::Owned(
                MessyJsonNumeric::new(MessyJsonNumberType::U64, !required),
            ))),
            BasiliqDbScannerTypeCategory::String => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Timespan => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::UserDefined => match type_.name().as_str() {
                "uuid" => Some(MessyJson::Uuid(Cow::Owned(MessyJsonScalar::new(!required)))),
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
            BasiliqDbScannerTypeCategory::BitString => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Boolean => {
                Some(MessyJson::Bool(Cow::Owned(MessyJsonScalar::new(!required))))
            }
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
    ) -> Option<BasiliqStoreTableBuilder<'a>> {
        let mut obj_properties: BTreeMap<String, MessyJson> = BTreeMap::new();
        let mut pkey_type: Option<CibouletteIdType> = None;
        trace!(
            "Scanning table {} in schema {}",
            table.table().name(),
            table.schema().name()
        );

        for (id, col_settings) in table.columns_by_id() {
            if pkey == *id {
                // If the primary key
                pkey_type = Self::type_to_id(col_settings);
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
            .map(|x| (x, MessyJsonObject::new(obj_properties, false)))
            .map(|(id_type, properties)| BasiliqStoreTableBuilder {
                table,
                properties,
                id_type,
            })
    }
}
