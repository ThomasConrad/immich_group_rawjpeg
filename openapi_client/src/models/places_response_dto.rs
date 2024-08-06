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
pub struct PlacesResponseDto {
    #[serde(rename = "admin1name", skip_serializing_if = "Option::is_none")]
    pub admin1name: Option<String>,
    #[serde(rename = "admin2name", skip_serializing_if = "Option::is_none")]
    pub admin2name: Option<String>,
    #[serde(rename = "latitude")]
    pub latitude: f64,
    #[serde(rename = "longitude")]
    pub longitude: f64,
    #[serde(rename = "name")]
    pub name: String,
}

impl PlacesResponseDto {
    pub fn new(latitude: f64, longitude: f64, name: String) -> PlacesResponseDto {
        PlacesResponseDto {
            admin1name: None,
            admin2name: None,
            latitude,
            longitude,
            name,
        }
    }
}

