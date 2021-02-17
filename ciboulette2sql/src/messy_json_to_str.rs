use super::*;
use quaint::ast::ExpressionKind;
use quaint::ast::Value as QuaintValue;

#[inline]
fn convert_messy_json_to_str_inner<'a>(
    val: MessyJsonValue<'a>,
) -> Result<QuaintValue<'a>, Ciboulette2SqlError> {
    Ok(match val {
        MessyJsonValue::Bool(val) => QuaintValue::Boolean(Some(val)),
        MessyJsonValue::Null(schema) => match schema {
            MessyJson::Bool(_) => QuaintValue::Boolean(None),
            MessyJson::Number(_) => QuaintValue::Numeric(None),
            MessyJson::String(_) => QuaintValue::Text(None),
            MessyJson::Array(_) => QuaintValue::Array(None),
            MessyJson::Obj(_) => unimplemented!(),
        },
        MessyJsonValue::Number(val) => QuaintValue::Numeric(Some(
            bigdecimal::FromPrimitive::from_u128(val)
                .ok_or_else(|| Ciboulette2SqlError::BigDecimal(val))?,
        )),
        MessyJsonValue::String(val) => QuaintValue::Text(Some(val)),
        MessyJsonValue::Array(arr) => {
            let mut arr_res: Vec<QuaintValue<'_>> = Vec::with_capacity(arr.len());
            for el in arr.take().into_iter() {
                arr_res.push(convert_messy_json_to_str_inner(el)?)
            }
            QuaintValue::Array(Some(arr_res))
        }
        MessyJsonValue::Obj(_obj) => {
            unimplemented!()
        }
    })
}

pub fn convert_messy_json_to_str<'a>(
    val: MessyJsonObjectValue<'a>,
) -> Result<Vec<(Cow<'a, str>, QuaintValue<'a>)>, Ciboulette2SqlError> {
    let mut res: Vec<(Cow<'a, str>, QuaintValue<'a>)> = Vec::with_capacity(val.len());

    for (k, v) in val.take().into_iter() {
        res.push((k, convert_messy_json_to_str_inner(v)?));
    }
    Ok(res)
}
