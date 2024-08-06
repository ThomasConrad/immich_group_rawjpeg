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
pub struct SmartInfoResponseDto {
    #[serde(rename = "objects", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub objects: Option<Option<Vec<String>>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<String>>>,
}

impl SmartInfoResponseDto {
    pub fn new() -> SmartInfoResponseDto {
        SmartInfoResponseDto {
            objects: None,
            tags: None,
        }
    }
}

