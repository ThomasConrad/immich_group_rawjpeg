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
pub struct SystemConfigMapDto {
    #[serde(rename = "darkStyle")]
    pub dark_style: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "lightStyle")]
    pub light_style: String,
}

impl SystemConfigMapDto {
    pub fn new(dark_style: String, enabled: bool, light_style: String) -> SystemConfigMapDto {
        SystemConfigMapDto {
            dark_style,
            enabled,
            light_style,
        }
    }
}

