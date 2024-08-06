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
pub struct UpdateAlbumUserDto {
    #[serde(rename = "role")]
    pub role: models::AlbumUserRole,
}

impl UpdateAlbumUserDto {
    pub fn new(role: models::AlbumUserRole) -> UpdateAlbumUserDto {
        UpdateAlbumUserDto {
            role,
        }
    }
}

