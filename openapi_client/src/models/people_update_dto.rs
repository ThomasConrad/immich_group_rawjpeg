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
pub struct PeopleUpdateDto {
    #[serde(rename = "people")]
    pub people: Vec<models::PeopleUpdateItem>,
}

impl PeopleUpdateDto {
    pub fn new(people: Vec<models::PeopleUpdateItem>) -> PeopleUpdateDto {
        PeopleUpdateDto {
            people,
        }
    }
}

