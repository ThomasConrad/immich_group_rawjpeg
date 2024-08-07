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
pub enum VideoContainer {
    #[serde(rename = "mov")]
    Mov,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "ogg")]
    Ogg,
    #[serde(rename = "webm")]
    Webm,

}

impl std::fmt::Display for VideoContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mov => write!(f, "mov"),
            Self::Mp4 => write!(f, "mp4"),
            Self::Ogg => write!(f, "ogg"),
            Self::Webm => write!(f, "webm"),
        }
    }
}

impl Default for VideoContainer {
    fn default() -> VideoContainer {
        Self::Mov
    }
}

