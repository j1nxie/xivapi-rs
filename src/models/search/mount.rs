use super::Metadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    #[serde(flatten)]
    pub metadata: Metadata,
}
