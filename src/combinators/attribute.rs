use nom::branch::alt;
use nom::combinator::opt;
use nom::sequence::separated_pair;
use nom::{character::complete::multispace0, multi::many0, IResult};

use crate::combinators::{comment, key, vdf_value_block, vdf_value_string};
use crate::{VdfAttribute, VdfValue};

pub fn attribute(input: &str) -> IResult<&str, VdfAttribute> {
    let (input, comments_before) = many0(comment)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, key_value) = opt(separated_pair(
        key,
        multispace0,
        alt((vdf_value_string, vdf_value_block)),
    ))(input)?;

    let (input, comment_after) = opt(comment)(input)?;

    if (comments_before.is_empty() && comment_after.is_none()) && key_value.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let attribute = match key_value {
        Some((key, value)) => VdfAttribute {
            comments_before,
            comment_after,
            key: key.to_string(),
            value,
        },
        None => VdfAttribute {
            comments_before,
            comment_after,
            key: "".to_string(),
            value: VdfValue::String("".to_string()),
        },
    };

    Ok((input, attribute))
}
