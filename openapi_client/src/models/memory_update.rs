/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.111.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryUpdate {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl MemoryUpdate {
    pub fn new() -> MemoryUpdate {
        MemoryUpdate {
            enabled: None,
        }
    }
}

