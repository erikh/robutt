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
pub struct GamesByGameIdAllOf {
    #[serde(rename = "data")]
    pub data: crate::models::GamesByGameIdAllOfData,
    #[serde(rename = "include")]
    pub include: crate::models::GamesByGameIdAllOfInclude,
}

impl GamesByGameIdAllOf {
    pub fn new(data: crate::models::GamesByGameIdAllOfData, include: crate::models::GamesByGameIdAllOfInclude) -> GamesByGameIdAllOf {
        GamesByGameIdAllOf {
            data,
            include,
        }
    }
}


