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
pub struct CreateProfileImageResponseDto {
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl CreateProfileImageResponseDto {
    pub fn new(profile_image_path: String, user_id: String) -> CreateProfileImageResponseDto {
        CreateProfileImageResponseDto {
            profile_image_path,
            user_id,
        }
    }
}

