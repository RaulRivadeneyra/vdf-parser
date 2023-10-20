use nom::character::complete::line_ending;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, not_line_ending},
    sequence::delimited,
    IResult,
};

pub fn comment(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, comment) = delimited(tag("//"), not_line_ending, line_ending)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, comment.trim().to_string()))
}

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    use super::comment;

    const TEST_RUNS: usize = 1000;
    #[test]
    fn any_comment() {
        // Run the test a bunch of times using randomly generated strings
        for _ in 0..TEST_RUNS {
            let random_string = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(rand::thread_rng().gen_range(1..100))
                .map(char::from)
                .collect::<String>();
            let input = format!("//{}\n", random_string);
            let result = comment(&input);
            // check if the result is Ok
            assert!(result.is_ok(), "Error: {:#?}", result);
            // check if the result is the same as the input
            assert_eq!(result.unwrap(), ("", random_string));
        }
    }

    #[test]
    fn contains_comment() {
        assert!(comment("//").is_err());
        assert!(comment("// ").is_err());
        assert!(comment("//\t").is_err());
        assert!(comment("//\\\"").is_err());
    }
}
