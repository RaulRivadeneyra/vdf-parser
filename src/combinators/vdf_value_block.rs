use std::collections::HashMap;

use nom::character::complete::char;
use nom::{character::complete::multispace0, multi::many0, sequence::delimited, IResult};

use crate::combinators::{block_content, comment};
use crate::VdfValue;

pub fn vdf_value_block(input: &str) -> IResult<&str, VdfValue> {
    let (input, _) = many0(comment)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, attributes) = delimited(char('{'), block_content, char('}'))(input)?;
    let (input, _) = multispace0(input)?;
    let block = attributes
        .into_iter()
        .map(|attribute| (attribute.key.clone(), attribute))
        .collect::<HashMap<_, _>>();
    Ok((input, VdfValue::Block(block)))
}
