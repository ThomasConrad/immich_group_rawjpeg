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
pub struct LicenseResponseDto {
    #[serde(rename = "activatedAt")]
    pub activated_at: String,
    #[serde(rename = "activationKey")]
    pub activation_key: String,
    #[serde(rename = "licenseKey")]
    pub license_key: String,
}

impl LicenseResponseDto {
    pub fn new(activated_at: String, activation_key: String, license_key: String) -> LicenseResponseDto {
        LicenseResponseDto {
            activated_at,
            activation_key,
            license_key,
        }
    }
}

