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
pub struct SystemConfigNewVersionCheckDto {
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl SystemConfigNewVersionCheckDto {
    pub fn new(enabled: bool) -> SystemConfigNewVersionCheckDto {
        SystemConfigNewVersionCheckDto {
            enabled,
        }
    }
}

