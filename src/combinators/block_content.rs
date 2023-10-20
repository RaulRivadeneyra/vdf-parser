use nom::{character::complete::multispace0, multi::many0, IResult};

use crate::combinators::attribute;
use crate::VdfAttribute;

pub fn block_content(input: &str) -> IResult<&str, Vec<VdfAttribute>> {
    let (input, _) = multispace0(input)?;
    let (input, content) = many0(attribute)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, content))
}
