use nom::branch::alt;
use nom::{bytes::complete::tag, character::complete::multispace0, IResult};

use crate::combinators::key;
use crate::VdfValue;

pub fn vdf_value_string(input: &str) -> IResult<&str, VdfValue> {
    let (input, _) = multispace0(input)?;
    let (input, value) = alt((key, tag("\"\"")))(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, VdfValue::String(value.to_string())))
}
