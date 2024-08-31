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
pub struct AssetIdsDto {
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<uuid::Uuid>,
}

impl AssetIdsDto {
    pub fn new(asset_ids: Vec<uuid::Uuid>) -> AssetIdsDto {
        AssetIdsDto {
            asset_ids,
        }
    }
}

