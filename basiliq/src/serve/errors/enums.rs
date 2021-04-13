use casey::snake;
use thiserror::Error;

macro_rules! error_id {
	($( { $name:ident, $code:literal } ),+) => {
		/// An enumeration of error ids to send to clients.
		///
		/// Every error in the 10000-19999 are client errors like: bad request, bad content type, utf8 errors and so on
		/// Every error in the 20000-29999 are JSON:API errors
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
						$name => {
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
						$name => {
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
    { Utf8,															"E10003" },
    { BadUrl,														"E10004" },
    { BadJson,														"E10005" },
    { BadUuid,														"E10006" },
    { BadNumber,													"E10007" },
    { BadUrlEncoded,												"E10008" },
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
    { CibouletteUnknownError,										"E20028" }
);
