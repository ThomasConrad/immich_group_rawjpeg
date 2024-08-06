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
pub struct OnThisDayDto {
    #[serde(rename = "year")]
    pub year: f64,
}

impl OnThisDayDto {
    pub fn new(year: f64) -> OnThisDayDto {
        OnThisDayDto {
            year,
        }
    }
}

