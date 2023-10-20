use nom::combinator::opt;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    IResult,
};

use crate::VdfValue;

pub fn vdf_value_string(input: &str) -> IResult<&str, VdfValue> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, value) = opt(is_not("\""))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, VdfValue::String(value.unwrap_or("").to_string())))
}

#[cfg(test)]
mod tests {
    use crate::VdfValue;

    use super::vdf_value_string;

    #[test]
    fn any_string() {
        let input = "\"test\"";
        let result = vdf_value_string(input);
        assert!(result.is_ok(), "Error: {:#?}", result);
        assert_eq!(result.unwrap(), ("", VdfValue::String("test".to_string())));
    }

    #[test]
    fn allows_empty_string() {
        let input = "\"\"";
        let result = vdf_value_string(input);
        assert!(result.is_ok(), "Error: {:#?}", result);
        assert_eq!(result.unwrap(), ("", VdfValue::String("".to_string())));
    }
}
