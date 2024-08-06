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
pub struct MapMarkerResponseDto {
    #[serde(rename = "city", deserialize_with = "Option::deserialize")]
    pub city: Option<String>,
    #[serde(rename = "country", deserialize_with = "Option::deserialize")]
    pub country: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lat")]
    pub lat: f64,
    #[serde(rename = "lon")]
    pub lon: f64,
    #[serde(rename = "state", deserialize_with = "Option::deserialize")]
    pub state: Option<String>,
}

impl MapMarkerResponseDto {
    pub fn new(city: Option<String>, country: Option<String>, id: String, lat: f64, lon: f64, state: Option<String>) -> MapMarkerResponseDto {
        MapMarkerResponseDto {
            city,
            country,
            id,
            lat,
            lon,
            state,
        }
    }
}

