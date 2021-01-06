SELECT
	pg_catalog.pg_constraint.oid as c_id,
	pg_catalog.pg_constraint.connamespace as c_schema,
	pg_catalog.pg_constraint.conname as c_name,
	pg_catalog.pg_constraint.conrelid as c_table_id,
	pg_catalog.pg_constraint.contypid as c_type_id,
	pg_catalog.pg_constraint.confrelid as c_fk_id,
	pg_catalog.pg_constraint.conkey as c_cols,
	pg_catalog.pg_constraint.confkey as c_fk_cols,
	pg_catalog.pg_constraint.conpfeqop as c_fk_eq,
	(SELECT pg_get_constraintdef(pg_catalog.pg_constraint.oid))
FROM pg_catalog.pg_constraint;
