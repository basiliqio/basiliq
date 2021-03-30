SELECT
	pg_catalog.pg_constraint.oid AS id,
    pg_catalog.pg_constraint.conname AS name,
    pg_catalog.pg_constraint.connamespace AS schema,
    pg_catalog.pg_constraint.conrelid AS table,
    pg_catalog.pg_constraint.conindid AS index,
	pg_catalog.pg_constraint.confrelid AS ftable,
	pg_catalog.pg_constraint.conkey AS lcolumns,
	pg_catalog.pg_constraint.confkey AS fcolumns
FROM pg_catalog.pg_constraint
WHERE
	contype = 'f';
