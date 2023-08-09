use super::Metadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
    #[serde(flatten)]
    pub metadata: Metadata,
}
