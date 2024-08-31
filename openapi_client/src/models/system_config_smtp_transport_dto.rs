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
pub struct SystemConfigSmtpTransportDto {
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "ignoreCert")]
    pub ignore_cert: bool,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "port")]
    pub port: f64,
    #[serde(rename = "username")]
    pub username: String,
}

impl SystemConfigSmtpTransportDto {
    pub fn new(host: String, ignore_cert: bool, password: String, port: f64, username: String) -> SystemConfigSmtpTransportDto {
        SystemConfigSmtpTransportDto {
            host,
            ignore_cert,
            password,
            port,
            username,
        }
    }
}

