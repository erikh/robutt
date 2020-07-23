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
pub struct GamesUpdates {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "remaining_monthly_allowance")]
    pub remaining_monthly_allowance: i32,
    #[serde(rename = "extra_allowance")]
    pub extra_allowance: i32,
    #[serde(rename = "pages")]
    pub pages: crate::models::PaginatedApiResponseAllOfPages,
    #[serde(rename = "data")]
    pub data: crate::models::GamesUpdatesAllOfData,
}

impl GamesUpdates {
    pub fn new(code: i32, status: String, remaining_monthly_allowance: i32, extra_allowance: i32, pages: crate::models::PaginatedApiResponseAllOfPages, data: crate::models::GamesUpdatesAllOfData) -> GamesUpdates {
        GamesUpdates {
            code,
            status,
            remaining_monthly_allowance,
            extra_allowance,
            pages,
            data,
        }
    }
}


