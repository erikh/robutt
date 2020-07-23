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
pub struct GamesUpdatesAllOf {
    #[serde(rename = "data")]
    pub data: crate::models::GamesUpdatesAllOfData,
}

impl GamesUpdatesAllOf {
    pub fn new(data: crate::models::GamesUpdatesAllOfData) -> GamesUpdatesAllOf {
        GamesUpdatesAllOf {
            data,
        }
    }
}


