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
pub struct CreateAlbumDto {
    #[serde(rename = "albumName")]
    pub album_name: String,
    #[serde(rename = "albumUsers", skip_serializing_if = "Option::is_none")]
    pub album_users: Option<Vec<models::AlbumUserCreateDto>>,
    #[serde(rename = "assetIds", skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateAlbumDto {
    pub fn new(album_name: String) -> CreateAlbumDto {
        CreateAlbumDto {
            album_name,
            album_users: None,
            asset_ids: None,
            description: None,
        }
    }
}

