SELECT
	pg_catalog.pg_constraint.oid AS id,
    pg_catalog.pg_constraint.conname AS name,
	main_schema.nspname as schema,
    main_table.relname AS table,
	pg_catalog.pg_constraint.conrelid AS table_id,
	foreign_schema.nspname as fschema,
	foreign_table.relname AS ftable,
	pg_catalog.pg_constraint.confrelid AS ftable_id,
	pg_catalog.pg_constraint.conkey AS lcolumns,
	pg_catalog.pg_constraint.confkey AS fcolumns
FROM pg_catalog.pg_constraint
INNER JOIN 
	pg_catalog.pg_class main_table ON main_table.oid = pg_catalog.pg_constraint.conrelid
INNER JOIN 
	pg_catalog.pg_namespace main_schema ON main_schema.oid = main_table.relnamespace
INNER JOIN 
	pg_catalog.pg_class foreign_table ON foreign_table.oid = pg_catalog.pg_constraint.confrelid
INNER JOIN 
	pg_catalog.pg_namespace foreign_schema ON foreign_schema.oid = foreign_table.relnamespace
WHERE
	contype = 'f';
