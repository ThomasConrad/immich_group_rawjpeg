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
pub struct SessionResponseDto {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "current")]
    pub current: bool,
    #[serde(rename = "deviceOS")]
    pub device_os: String,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl SessionResponseDto {
    pub fn new(created_at: String, current: bool, device_os: String, device_type: String, id: String, updated_at: String) -> SessionResponseDto {
        SessionResponseDto {
            created_at,
            current,
            device_os,
            device_type,
            id,
            updated_at,
        }
    }
}

