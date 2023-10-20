use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char};
use nom::multi::fold_many1;
use nom::{sequence::delimited, IResult};

/// ## Parses a key within quotes
/// _______
/// **Rules:**
/// * The key must be within quotes
/// * There must be at least one alphanumeric character within the quotes
pub fn key(input: &str) -> IResult<&str, &str> {
    let (input, key) = delimited(
        char('"'),
        fold_many1(
            alt((alphanumeric1, tag("_"), tag("-"))),
            String::new,
            |mut acc, item| {
                acc.push_str(item);
                acc
            },
        ),
        char('"'),
    )(input)?;

    Ok((input, key.leak()))
}

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    use super::key;

    const TEST_RUNS: usize = 1000;
    #[test]
    fn any_alphanumeric_within_quotes() {
        // Run the test a bunch of times using randomly generated strings
        for _ in 0..TEST_RUNS {
            let random_string = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(rand::thread_rng().gen_range(1..100))
                .map(char::from)
                .collect::<String>();
            let input = format!("\"{}\"", random_string);
            let result = key(&input);
            // check if the result is Ok
            assert!(result.is_ok(), "Error: {:#?}", result);
            // check if the result is the same as the input
            assert_eq!(result.unwrap(), ("", random_string.as_str()));
        }
    }

    #[test]
    fn contains_alphanumeric_within_quotes() {
        assert!(key("\"\"").is_err());
        assert!(key("\" \"").is_err());
        assert!(key("\"\t\"").is_err());
        assert!(key("\"\\\"\"").is_err());
    }
}
