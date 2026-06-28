use crate::types::BlockType;
use serde_json::{Value, Map};

pub fn validate_block_attributes(block: &BlockType, attributes: &Map<String, Value>) -> Result<(), String> {
    for (key, val) in attributes {
        if let Some(attr_schema) = block.attributes.get(key) {
            let is_valid = match attr_schema.attribute_type.as_str() {
                "string" => val.is_string(),
                "number" | "integer" => val.is_number(),
                "boolean" => val.is_boolean(),
                "object" => val.is_object(),
                "array" => val.is_array(),
                _ => true,
            };
            if !is_valid {
                return Err(format!("Attribute '{}' does not match type '{}'", key, attr_schema.attribute_type));
            }
        }
    }
    Ok(())
}
