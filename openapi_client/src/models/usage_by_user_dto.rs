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
pub struct UsageByUserDto {
    #[serde(rename = "photos")]
    pub photos: i32,
    #[serde(rename = "quotaSizeInBytes", deserialize_with = "Option::deserialize")]
    pub quota_size_in_bytes: Option<i64>,
    #[serde(rename = "usage")]
    pub usage: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "videos")]
    pub videos: i32,
}

impl UsageByUserDto {
    pub fn new(photos: i32, quota_size_in_bytes: Option<i64>, usage: i64, user_id: String, user_name: String, videos: i32) -> UsageByUserDto {
        UsageByUserDto {
            photos,
            quota_size_in_bytes,
            usage,
            user_id,
            user_name,
            videos,
        }
    }
}

