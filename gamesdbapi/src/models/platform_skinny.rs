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
pub struct PlatformSkinny {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "alias")]
    pub alias: String,
}

impl PlatformSkinny {
    pub fn new(id: i32, name: String, alias: String) -> PlatformSkinny {
        PlatformSkinny {
            id,
            name,
            alias,
        }
    }
}

