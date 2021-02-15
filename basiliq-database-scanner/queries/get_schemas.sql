SELECT
	oid as id,
	nspname as name,
	nspowner as owner,
	COALESCE((SELECT has_schema_privilege(pg_catalog.pg_namespace.nspname, 'USAGE')), false) as usage
	FROM pg_catalog.pg_namespace;

