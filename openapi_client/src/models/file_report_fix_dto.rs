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
pub struct FileReportFixDto {
    #[serde(rename = "items")]
    pub items: Vec<models::FileReportItemDto>,
}

impl FileReportFixDto {
    pub fn new(items: Vec<models::FileReportItemDto>) -> FileReportFixDto {
        FileReportFixDto {
            items,
        }
    }
}

