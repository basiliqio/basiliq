SELECT
	pg_catalog.pg_class.oid as id,
	pg_catalog.pg_class.relname as name,
	pg_catalog.pg_class.relnamespace as schema,
	pg_catalog.pg_class.reltype as type_,
	pg_catalog.pg_class.relowner as owner,
	pg_catalog.pg_class.relkind as kind,
	COALESCE(
		(
			CASE WHEN pg_catalog.pg_class.relkind = 'f' 
			THEN (has_server_privilege(pg_catalog.pg_class.oid, 'USAGE'))
			END
		),
		true) as usage_perm,
	has_table_privilege(pg_catalog.pg_class.oid, 'SELECT') as select_perm,
	has_table_privilege(pg_catalog.pg_class.oid, 'INSERT') as insert_perm,
	has_table_privilege(pg_catalog.pg_class.oid, 'UPDATE') as update_perm,
	has_table_privilege(pg_catalog.pg_class.oid, 'DELETE') as delete_perm
	FROM pg_catalog.pg_class
	WHERE
	relkind IN ('r', 't', 'v', 'c', 'f')
	AND
	relpersistence = 'p';
