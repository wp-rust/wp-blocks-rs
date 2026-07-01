use crate::types::BlockType;
use serde_json::{Value, Map};

pub fn validate_block_attributes(block: &BlockType, attributes: &Map<String, Value>) -> Result<(), String> {
    for (key, val) in attributes {
        if let Some(attr_schema) = block.attributes.get(key) {
            let mut valid = false;
            let expected_types = if let Some(arr) = attr_schema.attribute_type.as_array() {
                arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>()
            } else if let Some(s) = attr_schema.attribute_type.as_str() {
                vec![s]
            } else {
                vec![]
            };

            for exp in &expected_types {
                let match_type = match *exp {
                    "string" => val.is_string(),
                    "number" | "integer" => val.is_number(),
                    "boolean" => val.is_boolean(),
                    "object" => val.is_object(),
                    "array" => val.is_array(),
                    "null" => val.is_null(),
                    _ => true,
                };
                if match_type {
                    valid = true;
                    break;
                }
            }

            if !valid && !expected_types.is_empty() {
                return Err(format!("Attribute '{}' does not match any of types: {:?}", key, expected_types));
            }
        }
    }
    Ok(())
}
