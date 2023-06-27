use super::Metadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    #[serde(flatten)]
    pub metadata: Metadata,
    pub description: String,
}
