use super::*;
use log::trace;
const POSTGRES_SYSTEM_COLUMNS: &[&str] =
    &["oid", "tableoid", "xmin", "cmin", "xmax", "cmax", "ctid"];

const POSTGRES_SYSTEM_SCHEMA: &[&str] = &["pg_catalog", "pg_toast", "information_schema"];

impl BasiliqStoreBuilder {
    fn type_to_messy_json<'a>(
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
            BasiliqDbScannerTypeCategory::Enum => {
                trace!(
                    ">> Found an enum for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::Geo => {
                trace!(
                    ">> Found an geo for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::NetworkAddress => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Pseudo => {
                trace!(
                    ">> Found a pseudo- for type {}type. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::Range => {
                trace!(
                    ">> Found a range for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::DateTime => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Numeric => Some(MessyJson::Number(Cow::Owned(
                MessyJsonNumeric::new(MessyJsonNumberType::U64, !required),
            ))),
            BasiliqDbScannerTypeCategory::String => Some(MessyJson::String(Cow::Owned(
                MessyJsonScalar::new(!required),
            ))),
            BasiliqDbScannerTypeCategory::Timespan => {
                trace!(
                    ">> Found a timespan for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
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
            BasiliqDbScannerTypeCategory::BitString => {
                trace!(
                    ">> Found a bitstring for type {}. Unsupported, skipping..",
                    type_.name()
                );
                None
            }
            BasiliqDbScannerTypeCategory::Boolean => {
                Some(MessyJson::Bool(Cow::Owned(MessyJsonScalar::new(!required))))
            }
        }
    }

    pub fn build_object(&self) -> BTreeMap<String, MessyJsonObject> {
        let mut res: BTreeMap<String, MessyJsonObject> = BTreeMap::new();

        'table: for table in self.tables.iter() {
            let mut obj_properties: BTreeMap<String, MessyJson> = BTreeMap::new();
            if POSTGRES_SYSTEM_SCHEMA.contains(&table.schema().name().as_str())
            // If in a system schema
            {
                continue 'table;
            }
            trace!(
                "Scanning table {} in schema {}",
                table.table().name(),
                table.schema().name()
            );

            'col: for col_settings in table.columns_by_id().values() {
                if POSTGRES_SYSTEM_COLUMNS.contains(&col_settings.column().name().as_str()) {
                    // If a system column
                    continue 'col;
                }
                trace!("> Scanning columns {}", col_settings.column().name());
                if let Some(obj) = match col_settings.type_() {
                    BasiliqDbScannerType::Simple(type_) => {
                        Self::type_to_messy_json(col_settings, type_)
                    }
                    BasiliqDbScannerType::Nested(_parent, _child) => {
                        trace!("Found an array. Unsupported, skipping..");
                        None
                    }
                } {
                    obj_properties.insert(col_settings.column().name().clone(), obj);
                }
            }
            res.insert(
                table.table().name().clone(),
                MessyJsonObject::new(obj_properties, false),
            );
        }
        res
    }
}
