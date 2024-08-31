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
pub struct AdminOnboardingUpdateDto {
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
}

impl AdminOnboardingUpdateDto {
    pub fn new(is_onboarded: bool) -> AdminOnboardingUpdateDto {
        AdminOnboardingUpdateDto {
            is_onboarded,
        }
    }
}

