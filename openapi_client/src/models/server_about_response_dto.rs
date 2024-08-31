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
pub struct ServerAboutResponseDto {
    #[serde(rename = "build", skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(rename = "buildImage", skip_serializing_if = "Option::is_none")]
    pub build_image: Option<String>,
    #[serde(rename = "buildImageUrl", skip_serializing_if = "Option::is_none")]
    pub build_image_url: Option<String>,
    #[serde(rename = "buildUrl", skip_serializing_if = "Option::is_none")]
    pub build_url: Option<String>,
    #[serde(rename = "exiftool", skip_serializing_if = "Option::is_none")]
    pub exiftool: Option<String>,
    #[serde(rename = "ffmpeg", skip_serializing_if = "Option::is_none")]
    pub ffmpeg: Option<String>,
    #[serde(rename = "imagemagick", skip_serializing_if = "Option::is_none")]
    pub imagemagick: Option<String>,
    #[serde(rename = "libvips", skip_serializing_if = "Option::is_none")]
    pub libvips: Option<String>,
    #[serde(rename = "licensed")]
    pub licensed: bool,
    #[serde(rename = "nodejs", skip_serializing_if = "Option::is_none")]
    pub nodejs: Option<String>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "repositoryUrl", skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(rename = "sourceCommit", skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
    #[serde(rename = "sourceRef", skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    #[serde(rename = "sourceUrl", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "versionUrl")]
    pub version_url: String,
}

impl ServerAboutResponseDto {
    pub fn new(licensed: bool, version: String, version_url: String) -> ServerAboutResponseDto {
        ServerAboutResponseDto {
            build: None,
            build_image: None,
            build_image_url: None,
            build_url: None,
            exiftool: None,
            ffmpeg: None,
            imagemagick: None,
            libvips: None,
            licensed,
            nodejs: None,
            repository: None,
            repository_url: None,
            source_commit: None,
            source_ref: None,
            source_url: None,
            version,
            version_url,
        }
    }
}

