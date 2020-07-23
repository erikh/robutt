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
pub struct Developers {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "remaining_monthly_allowance")]
    pub remaining_monthly_allowance: i32,
    #[serde(rename = "extra_allowance")]
    pub extra_allowance: i32,
    #[serde(rename = "allowance_refresh_timer", skip_serializing_if = "Option::is_none")]
    pub allowance_refresh_timer: Option<i32>,
    #[serde(rename = "data")]
    pub data: crate::models::DevelopersAllOfData,
}

impl Developers {
    pub fn new(code: i32, status: String, remaining_monthly_allowance: i32, extra_allowance: i32, data: crate::models::DevelopersAllOfData) -> Developers {
        Developers {
            code,
            status,
            remaining_monthly_allowance,
            extra_allowance,
            allowance_refresh_timer: None,
            data,
        }
    }
}


