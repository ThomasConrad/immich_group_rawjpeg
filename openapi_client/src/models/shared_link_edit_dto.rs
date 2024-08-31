/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.113.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedLinkEditDto {
    #[serde(rename = "allowDownload", skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(rename = "allowUpload", skip_serializing_if = "Option::is_none")]
    pub allow_upload: Option<bool>,
    /// Few clients cannot send null to set the expiryTime to never. Setting this flag and not sending expiryAt is considered as null instead. Clients that can send null values can ignore this.
    #[serde(rename = "changeExpiryTime", skip_serializing_if = "Option::is_none")]
    pub change_expiry_time: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expiresAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Option<String>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "showMetadata", skip_serializing_if = "Option::is_none")]
    pub show_metadata: Option<bool>,
}

impl SharedLinkEditDto {
    pub fn new() -> SharedLinkEditDto {
        SharedLinkEditDto {
            allow_download: None,
            allow_upload: None,
            change_expiry_time: None,
            description: None,
            expires_at: None,
            password: None,
            show_metadata: None,
        }
    }
}

