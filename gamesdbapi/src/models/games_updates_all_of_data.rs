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
pub struct GamesUpdatesAllOfData {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "updates")]
    pub updates: Vec<crate::models::UpdateModel>,
}

impl GamesUpdatesAllOfData {
    pub fn new(count: i32, updates: Vec<crate::models::UpdateModel>) -> GamesUpdatesAllOfData {
        GamesUpdatesAllOfData {
            count,
            updates,
        }
    }
}


