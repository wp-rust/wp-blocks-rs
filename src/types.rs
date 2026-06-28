use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockAttribute {
    #[serde(rename = "type")]
    pub attribute_type: String,
    pub default: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSupport {
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockTransform {
    #[serde(rename = "type")]
    pub transform_type: String,
    pub blocks: Option<Vec<String>>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockVariation {
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
    pub attributes: Option<HashMap<String, Value>>,
    #[serde(rename = "innerBlocks")]
    pub inner_blocks: Option<Vec<Value>>,
    pub example: Option<Value>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockType {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "apiVersion")]
    pub api_version: Option<u32>,
    pub keywords: Option<Vec<String>>,
    pub textdomain: Option<String>,
    pub parent: Option<Vec<String>>,
    pub ancestor: Option<Vec<String>>,
    #[serde(rename = "allowedBlocks")]
    pub allowed_blocks: Option<Vec<String>>,
    #[serde(rename = "providesContext")]
    pub provides_context: Option<HashMap<String, String>>,
    #[serde(rename = "usesContext")]
    pub uses_context: Option<Vec<String>>,
    #[serde(default)]
    pub attributes: HashMap<String, BlockAttribute>,
    pub supports: Option<BlockSupport>,
    pub styles: Option<Vec<Value>>,
    pub example: Option<Value>,
    #[serde(default)]
    pub variations: Vec<BlockVariation>,
    #[serde(default)]
    pub transforms: HashMap<String, Vec<BlockTransform>>,
}
