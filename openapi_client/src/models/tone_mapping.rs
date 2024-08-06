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
pub enum ToneMapping {
    #[serde(rename = "hable")]
    Hable,
    #[serde(rename = "mobius")]
    Mobius,
    #[serde(rename = "reinhard")]
    Reinhard,
    #[serde(rename = "disabled")]
    Disabled,

}

impl std::fmt::Display for ToneMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hable => write!(f, "hable"),
            Self::Mobius => write!(f, "mobius"),
            Self::Reinhard => write!(f, "reinhard"),
            Self::Disabled => write!(f, "disabled"),
        }
    }
}

impl Default for ToneMapping {
    fn default() -> ToneMapping {
        Self::Hable
    }
}

