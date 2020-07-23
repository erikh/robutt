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
pub struct ImageBaseUrlMeta {
    #[serde(rename = "original")]
    pub original: String,
    #[serde(rename = "small")]
    pub small: String,
    #[serde(rename = "thumb")]
    pub thumb: String,
    #[serde(rename = "cropped_center_thumb")]
    pub cropped_center_thumb: String,
    #[serde(rename = "medium")]
    pub medium: String,
    #[serde(rename = "large")]
    pub large: String,
}

impl ImageBaseUrlMeta {
    pub fn new(original: String, small: String, thumb: String, cropped_center_thumb: String, medium: String, large: String) -> ImageBaseUrlMeta {
        ImageBaseUrlMeta {
            original,
            small,
            thumb,
            cropped_center_thumb,
            medium,
            large,
        }
    }
}


