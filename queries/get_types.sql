SELECT
	pg_catalog.pg_type.oid as id,
	pg_catalog.pg_type.typnamespace as schema,
	pg_catalog.pg_type.typname as name,
	(
		CASE WHEN pg_catalog.pg_type.typlen < 0
		THEN (NULL)
		ELSE (pg_catalog.pg_type.typlen)
		END
	) AS len,
	pg_catalog.pg_type.typtype as type_,
	(
		CASE WHEN pg_catalog.pg_type.typrelid = 0
		THEN (NULL)
		ELSE (pg_catalog.pg_type.typrelid)
		END
	) AS rel_id,
	(
		CASE WHEN pg_catalog.pg_type.typelem = 0
		THEN (NULL)
		ELSE (pg_catalog.pg_type.typelem)
		END
	) AS child_type,
	(
		CASE WHEN pg_catalog.pg_type.typarray = 0
		THEN (NULL)
		ELSE (pg_catalog.pg_type.typarray)
		END
	) AS parent_type,
	pg_catalog.pg_type.typcategory as category,
	pg_catalog.pg_type.typndims as dimensions
FROM pg_catalog.pg_type
WHERE pg_catalog.pg_type.typisdefined = true;
