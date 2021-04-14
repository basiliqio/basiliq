use casey::snake;

macro_rules! error_id {
	($( { $name:ident, $code:literal } ),+) => {
		/// An enumeration of error ids to send to clients.
		///
		/// Every error in the 10000-19999 are client errors like: bad request, bad content type, utf8 errors and so on
		/// Every error in the 20000-29999 are JSON:API (Ciboulette) errors
		/// Every error in the 30000-39999 are Ciboulette2Postgres errors
		/// Every error in the 50000-59999 are server errors (IO, etc)
		#[derive(Copy, Debug, Clone)]
		pub enum BasiliqErrorId
		{
			$(
				$name,
			)*
		}

		impl BasiliqErrorId
		{
			pub fn title(&self) -> &str
			{
				match self
				{
					$(
						BasiliqErrorId::$name => {
							snake!(stringify!($name))
						}
					)*
				}
			}

			pub fn id(&self) -> &str
			{
				match self
				{
					$(
						BasiliqErrorId::$name => {
							$code
						}
					)*
				}
			}
		}
	};
}

error_id!(
    { BadContentType,												"E10000" },
    { BadMethod,													"E10001" },
    { BadHeader,													"E10002" },
    { BadUrl,														"E10003" },
    { BadJson,														"E10004" },
    { BadUuid,														"E10005" },
    { BadNumber,													"E10006" },
    { BadUrlEncoded,												"E10007" },
    { BadBigNumber,													"E10008" },
    { ToUtf8,														"E10009" },
    { FromUtf8,														"E10010" },
    { BadRequest,													"E10011" },

    { CibouletteMainTypeClash,										"E20000" },
    { CibouletteUnknownType,										"E20001" },
    { CibouletteUnknownRelationship,								"E20002" },
    { CibouletteUnknownField,										"E20003" },
    { CibouletteIncompatibleSorting,								"E20004" },
    { CibouletteNestedSorting,										"E20005" },
    { CibouletteUniqObj,											"E20006" },
    { CibouletteUniqType,											"E20007" },
    { CibouletteUniqRelationshipObject,								"E20008" },
    { CibouletteUniqRelationship,									"E20008" },
    { CibouletteMissingLink,										"E20009" },
    { CibouletteNoCompleteLinkage,									"E20010" },
    { CibouletteTypeNotInGraph,										"E20011" },
    { CibouletteRelNotInGraph,										"E20012" },
    { CibouletteKeyClash,											"E20013" },
    { CibouletteInvalidMemberName,									"E20014" },
    { CibouletteAttributesIsNotAnObject,							"E20015" },
    { CibouletteEmptyQueryAttribute,								"E20016" },
    { CibouletteNoData,												"E20017" },
    { CibouletteMissingId,											"E20018" },
    { CibouletteBadIdType,											"E20019" },
    { CibouletteUnknownIdType,										"E20020" },
    { CibouletteNoCompound,											"E20021" },
    { CibouletteMissingAliasTranslation,							"E20022" },
    { CibouletteMissingTypeInPath,									"E20023" },
    { CibouletteBadPath,											"E20024" },
    { CibouletteWrongIntention,										"E20025" },
    { CibouletteWrongPathType,										"E20026" },
    { CibouletteOutboundTooManyMainData,							"E20027" },
    { CibouletteUnknownError,										"E20028" },

    { Ciboulette2PostgresMissingRelationship,						"E30000" },
    { Ciboulette2PostgresRequiredSingleRelationship,				"E30001" },
    { Ciboulette2PostgresUnknownTable,								"E30002" },
    { Ciboulette2PostgresEmptyRelValueError,						"E30003" },
    { Ciboulette2PostgresNullCharIdent,								"E30004" },
    { Ciboulette2PostgresUpdatingRelationships,						"E30005" },
    { Ciboulette2PostgresUpdatingManyRelationships,					"E30006" },
    { Ciboulette2PostgresUpdatingMainObject,						"E30007" },
    { Ciboulette2PostgresMultiIdsForSingleRelationships,			"E30008" },
    { Ciboulette2PostgresBulkRelationshipDelete,					"E30009" },
    { Ciboulette2PostgresMissingRelationForOrdering,				"E30010" },
    { Ciboulette2PostgresNonAsciiCharInIdent,						"E30011" },
    { Ciboulette2PostgresProvidedIdOnInserts,						"E30012" },
    { Ciboulette2PostgresMissingAttributes,							"E30013" },
    { Ciboulette2PostgresSortingByMultiRel,							"E30014" },
    { Ciboulette2PostgresUnknownError,								"E30015" },

    { Io,															"E50000" },
    { BufReaderInto,												"E50001" },
    { HttpError,													"E50002" },

    { UnknownError,													"E99999" }
);
