use std::collections::HashMap;

use crate::combinators::attribute;
use crate::error::VdfError;

mod combinators;
pub mod error;

#[derive(Debug, PartialEq)]
pub enum VdfValue {
    String(String),
    Block(HashMap<String, VdfAttribute>),
}

impl From<VdfValue> for String {
    fn from(value: VdfValue) -> Self {
        match value {
            VdfValue::String(string) => string,
            VdfValue::Block(_) => panic!("Cannot convert VdfValue::Block to String"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VdfAttribute {
    pub comments_before: Vec<String>,
    pub comment_after: Option<String>,
    pub key: String,
    pub value: VdfValue,
}

impl VdfValue {}

impl VdfAttribute {
    pub fn get_string_value(&self, key: &str) -> Result<&VdfAttribute, VdfError> {
        match &self.value {
            VdfValue::Block(block) => match block.get(key) {
                Some(value) => Ok(value),
                None => Err(VdfError::ValueNotFound(
                    key.to_string(),
                    self.key.to_string(),
                )),
            },
            _ => Err(VdfError::ValueNotFound(
                key.to_string(),
                self.key.to_string(),
            )),
        }
    }
}

/// ## Parse a VDF text
/// ______
/// - `text` - The VDF text to parse
/// - `return` - A `Result` containing either a `VdfAttribute` or a `VdfError`
pub fn parse_vdf_text(text: &str) -> Result<VdfAttribute, VdfError> {
    let (_, vdf) = attribute(text).expect("Failed to parse vdf file");
    Ok(vdf)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_vdf_text, VdfAttribute, VdfValue};

    #[test]
    fn it_works() {
        {
            let input = r#"
            // Comment at the start of the file
            "default_attack"
            {
                // Comment before a key-value pair 1
                // Comment before a key-value pair 2
                // Comment before a key-value pair 3
                "ID" "5001"
                "Damage" "100" // Comment after a key-value pair

                "test_block"
                {
                    "test_key" "test_value"
                }
            }
            // Comment at the end of the file
            "#;

            let result = parse_vdf_text(input);
            let expected = VdfAttribute {
                key: "default_attack".to_string(),
                comments_before: vec!["Comment at the start of the file".to_string()],
                comment_after: Some("Comment at the end of the file".to_string()),
                value: VdfValue::Block(HashMap::from([
                    (
                        "Damage".to_string(),
                        VdfAttribute {
                            key: "Damage".to_string(),
                            comments_before: vec![],
                            comment_after: Some("Comment after a key-value pair".to_string()),
                            value: VdfValue::String("100".to_string()),
                        },
                    ),
                    (
                        "ID".to_string(),
                        VdfAttribute {
                            key: "ID".to_string(),
                            comments_before: vec![
                                "Comment before a key-value pair 1".to_string(),
                                "Comment before a key-value pair 2".to_string(),
                                "Comment before a key-value pair 3".to_string(),
                            ],
                            comment_after: None,
                            value: VdfValue::String("5001".to_string()),
                        },
                    ),
                    (
                        "test_block".to_string(),
                        VdfAttribute {
                            key: "test_block".to_string(),
                            comments_before: vec![],
                            comment_after: None,
                            value: VdfValue::Block(HashMap::from([(
                                "test_key".to_string(),
                                VdfAttribute {
                                    key: "test_key".to_string(),
                                    comments_before: vec![],
                                    comment_after: None,
                                    value: VdfValue::String("test_value".to_string()),
                                },
                            )])),
                        },
                    ),
                ])),
            };
            assert!(result.is_ok(), "Failed to parse attribute");
            let result = result.unwrap();
            assert_eq!(result.key, expected.key, "Parsed key does not match");
            assert_eq!(result.value, expected.value, "Parsed value does not match");
        }
    }

    #[test]
    fn can_parse_multiple_nested_blocks() {
        let input = r#"
        "default_attack"
        {
            // Comment before a key-value pair 1
            // Comment before a key-value pair 2
            // Comment before a key-value pair 3
            "ID" "5001"
            "Damage" "100" // Comment after a key-value pair
            "test_block_a"
            {
                "test_key" "test_value"

                "test_block_a_a"
                {
                    "test_key2" "test_value2"
                }
            }
            "test_block_b"
            {
                "test_key2" "test_value2"
            }
        }
        "#;

        let result = parse_vdf_text(input);
        assert!(result.is_ok(), "Failed to parse attribute");
    }

    #[test]
    fn can_parse_block_with_only_comments() {
        let input = r#"
        "default_attack"
        {
            "ID" "5001"
            "Damage" "100"
            "test_block_a"
            {
                "test_key" "test_value"

                "test_block_a_a"
                {
                    //"test_key2" "test_value2"
                }
            }
            "test_block_b"
            {
                "test_key2" "test_value2"
            }
        }
        "#;
        let result = parse_vdf_text(input);
        assert!(result.is_ok(), "Failed to parse attribute");
    }

    #[test]
    fn can_parse_value_with_spaces() {
        let input = r#"
        "default_attack"
        {
            "ID" "5001"
            "Damage" "100"
            "test_block"
            {
                "test_key" "0 0 0 0 0"
            }
        }
        "#;

        let expected = VdfAttribute {
            key: "default_attack".to_string(),
            comments_before: vec![],
            comment_after: None,
            value: VdfValue::Block(HashMap::from([
                (
                    "Damage".to_string(),
                    VdfAttribute {
                        key: "Damage".to_string(),
                        comments_before: vec![],
                        comment_after: None,
                        value: VdfValue::String("100".to_string()),
                    },
                ),
                (
                    "ID".to_string(),
                    VdfAttribute {
                        key: "ID".to_string(),
                        comments_before: vec![],
                        comment_after: None,
                        value: VdfValue::String("5001".to_string()),
                    },
                ),
                (
                    "test_block".to_string(),
                    VdfAttribute {
                        key: "test_block".to_string(),
                        comments_before: vec![],
                        comment_after: None,
                        value: VdfValue::Block(HashMap::from([(
                            "test_key".to_string(),
                            VdfAttribute {
                                key: "test_key".to_string(),
                                comments_before: vec![],
                                comment_after: None,
                                value: VdfValue::String("0 0 0 0 0".to_string()),
                            },
                        )])),
                    },
                ),
            ])),
        };
        let result = parse_vdf_text(input);
        assert!(result.is_ok(), "Failed to parse attribute");
        let result = result.unwrap();
        assert_eq!(result, expected, "Parsed value does not match");
        println!("Result {:#?}", result);
        println!("Expected {:#?}", expected);
    }
}
