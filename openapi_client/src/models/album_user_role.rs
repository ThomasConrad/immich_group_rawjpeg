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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlbumUserRole {
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "viewer")]
    Viewer,

}

impl std::fmt::Display for AlbumUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Editor => write!(f, "editor"),
            Self::Viewer => write!(f, "viewer"),
        }
    }
}

impl Default for AlbumUserRole {
    fn default() -> AlbumUserRole {
        Self::Editor
    }
}

