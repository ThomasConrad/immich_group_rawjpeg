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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "ASSET")]
    Asset,
    #[serde(rename = "ALBUM")]
    Album,

}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Asset => write!(f, "ASSET"),
            Self::Album => write!(f, "ALBUM"),
        }
    }
}

impl Default for EntityType {
    fn default() -> EntityType {
        Self::Asset
    }
}

