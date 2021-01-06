SELECT
	pg_catalog.pg_attribute.attrelid as table,
	pg_catalog.pg_attribute.attname as name,
	pg_catalog.pg_attribute.atttypid as type_,
	pg_catalog.pg_attribute.attndims as dimensions,
	pg_catalog.pg_attribute.attnotnull as non_null,
	pg_catalog.pg_attribute.atthasdef as has_default,
	has_column_privilege(CURRENT_USER, pg_catalog.pg_attribute.attrelid, pg_catalog.pg_attribute.attname, 'INSERT') as insert_perm,
	has_column_privilege(CURRENT_USER, pg_catalog.pg_attribute.attrelid, pg_catalog.pg_attribute.attname, 'SELECT') as select_perm,
	has_column_privilege(CURRENT_USER, pg_catalog.pg_attribute.attrelid, pg_catalog.pg_attribute.attname, 'UPDATE') as update_perm,
	has_column_privilege(CURRENT_USER, pg_catalog.pg_attribute.attrelid, pg_catalog.pg_attribute.attname, 'REFERENCES') as reference_perm
FROM pg_catalog.pg_attribute;
