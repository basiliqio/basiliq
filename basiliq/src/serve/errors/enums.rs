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
    { CibouletteNoData,												"E20004" },
    { CibouletteMissingId,											"E20005" },
    { CibouletteNoCompound,											"E20006" },
    { CibouletteMissingTypeInPath,									"E20007" },
    { CibouletteBadPath,											"E20008" },
    { CibouletteWrongPathType,										"E20009" },
    { CibouletteUnknownError,										"E29999" },

    { Ciboulette2PostgresMissingRelationship,						"E30000" },
    { Ciboulette2PostgresRequiredSingleRelationship,				"E30001" },
    { Ciboulette2PostgresUnknownTable,								"E30002" },
    { Ciboulette2PostgresEmptyRelValueError,						"E30003" },
    { Ciboulette2PostgresNullCharIdent,								"E30004" },
    { Ciboulette2PostgresUpdatingMainObject,						"E30005" },
    { Ciboulette2PostgresManyRelationshipDirectWrite,				"E30006" },
    { Ciboulette2PostgresMultiIdsForSingleRelationships,			"E30007" },
    { Ciboulette2PostgresMissingRelationForSorting,					"E30008" },
    { Ciboulette2PostgresNonAsciiCharInIdent,						"E30009" },
    { Ciboulette2PostgresProvidedIdOnInserts,						"E30010" },
    { Ciboulette2PostgresMissingAttributes,							"E30011" },
    { Ciboulette2PostgresSortingByMultiRel,							"E30012" },
    { Ciboulette2PostgresUnknownError,								"E30013" },

    { Io,															"E50000" },
    { BufReaderInto,												"E50001" },
    { HttpError,													"E50002" },

    { UnknownError,													"E99999" }
);
