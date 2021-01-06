use super::*;
use getset::Getters;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqTable {
    schema: raw::PostgresSchemaRaw,
    table: raw::PostgresTableRaw,
    columns: HashMap<String, BasiliqColumns>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct BasiliqColumns {
    column: raw::PostgresColumnRaw,
    type_: BasiliqType,
}

#[derive(Debug, Clone)]
pub enum BasiliqType {
	Simple(raw::PostgresTypeRaw),
	Nested(raw::PostgresTypeRaw, Box<BasiliqType>)
}

impl BasiliqTable {
    pub fn new(
        schemas: Vec<raw::PostgresSchemaRaw>,
        tables: Vec<raw::PostgresTableRaw>,
        columns: Vec<raw::PostgresColumnRaw>,
		types: Vec<raw::PostgresTypeRaw>,
    ) -> Result<Vec<Self>> {
		let parsed_columns: Vec<BasiliqColumns> = BasiliqColumns::new(&types, &columns)?;
		let mut res: Vec<BasiliqTable> = Vec::with_capacity(tables.len());

        for table in tables.iter() {
			let schema_for_table: raw::PostgresSchemaRaw = BasiliqTable::find_schema_for_table(&schemas, &tables)
				.ok_or(anyhow!("The table {} doesn't belong to any schema", table.name()))?;
			let columns_for_table: HashMap<String, BasiliqColumns> = BasiliqTable::find_columns_for_table(&table, &parsed_columns)?;
			res.push(BasiliqTable{
				table: table.clone(),
				columns: columns_for_table,
				schema: schema_for_table,
			});
		}
		Ok(res)
    }

    fn find_schema_for_table(
        schemas: &Vec<raw::PostgresSchemaRaw>,
        tables: &Vec<raw::PostgresTableRaw>,
    ) -> Option<raw::PostgresSchemaRaw> {
        for table in tables.iter() {
            for schema in schemas.iter() {
                if schema.id() == table.schema() {
                    return Some(schema.clone());
                }
            }
        }
        None
	}

	fn find_columns_for_table(
		table: &raw::PostgresTableRaw,
		columns: &Vec<BasiliqColumns>,
	) -> Result<HashMap<String, BasiliqColumns>> {
		let mut res: HashMap<String, BasiliqColumns> = HashMap::new();
	
		for obj in columns.iter()
		{
			if obj.column().table() == table.id()
			{
				match res.insert(obj.column().name().clone(), obj.clone())
				{
					Some(_x) => bail!("Duplicate column {} for table {}", obj.column().name(), table.name()),
					None => ()
				};
			}
		}
		Ok(res)
	}
	
}

impl BasiliqType
{
	pub fn new(type_: raw::PostgresTypeRaw, list_available: &Vec<raw::PostgresTypeRaw>) -> Result<Self>
	{
		match type_.child_type()
		{
			Some(child_id) =>
			{
				for available_type in list_available.iter()
				{
					if available_type.id() == child_id
					{
						return Ok(BasiliqType::Nested(type_.clone(), Box::new(BasiliqType::new(type_, list_available)?)));
					}
				}
				bail!("Nested type {} not found for type {}", child_id, type_.id());
			},
			None => Ok(BasiliqType::Simple(type_))
		}
	}
}

impl BasiliqColumns {

	pub fn new(
		types: &Vec<raw::PostgresTypeRaw>,
		columns: &Vec<raw::PostgresColumnRaw>,
	) -> Result<Vec<Self>>
	{
		Ok(BasiliqColumns::find_types_for_columns(types, columns)?.into_iter().map(|(col, typ)|
		{
			BasiliqColumns {
				column: col,
				type_: typ
			}
		}).collect())
	}

	fn find_types_for_columns(
		types: &Vec<raw::PostgresTypeRaw>,
		columns: &Vec<raw::PostgresColumnRaw>,
	) -> Result<Vec<(raw::PostgresColumnRaw, BasiliqType)>> {
		let mut res: Vec<(raw::PostgresColumnRaw, BasiliqType)> = Vec::with_capacity(columns.len());
	
		for column in columns.iter()
		{
			for type_ in types.iter()
			{
				match type_.rel_id()
				{
					Some(id) => {
						if id == column.table()
						{
							res.push((column.clone(), BasiliqType::new(type_.clone(), &types)?));
						}
					},
					None => continue
				}
			}
		}
		Ok(res)
	}
}
