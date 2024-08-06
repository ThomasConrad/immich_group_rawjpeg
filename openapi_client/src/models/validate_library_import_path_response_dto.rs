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
pub struct ValidateLibraryImportPathResponseDto {
    #[serde(rename = "importPath")]
    pub import_path: String,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ValidateLibraryImportPathResponseDto {
    pub fn new(import_path: String, is_valid: bool) -> ValidateLibraryImportPathResponseDto {
        ValidateLibraryImportPathResponseDto {
            import_path,
            is_valid,
            message: None,
        }
    }
}

