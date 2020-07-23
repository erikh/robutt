/*
 * TheGamesDB API
 *
 * API Documentation
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformsByPlatformName {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "remaining_monthly_allowance")]
    pub remaining_monthly_allowance: i32,
    #[serde(rename = "extra_allowance")]
    pub extra_allowance: i32,
    #[serde(rename = "data")]
    pub data: crate::models::PlatformsByPlatformNameAllOfData,
}

impl PlatformsByPlatformName {
    pub fn new(code: i32, status: String, remaining_monthly_allowance: i32, extra_allowance: i32, data: crate::models::PlatformsByPlatformNameAllOfData) -> PlatformsByPlatformName {
        PlatformsByPlatformName {
            code,
            status,
            remaining_monthly_allowance,
            extra_allowance,
            data,
        }
    }
}

