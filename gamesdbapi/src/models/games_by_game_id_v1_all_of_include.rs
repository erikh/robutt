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
pub struct GamesByGameIdV1AllOfInclude {
    #[serde(rename = "boxart")]
    pub boxart: crate::models::GamesByGameIdAllOfIncludeBoxart,
    #[serde(rename = "platform")]
    pub platform: ::std::collections::HashMap<String, crate::models::PlatformSkinny>,
}

impl GamesByGameIdV1AllOfInclude {
    pub fn new(boxart: crate::models::GamesByGameIdAllOfIncludeBoxart, platform: ::std::collections::HashMap<String, crate::models::PlatformSkinny>) -> GamesByGameIdV1AllOfInclude {
        GamesByGameIdV1AllOfInclude {
            boxart,
            platform,
        }
    }
}


