use super::Metadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weather {
    #[serde(flatten)]
    pub metadata: Metadata,
    pub description: String,
}
