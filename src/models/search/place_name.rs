use super::Metadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceName {
    #[serde(flatten)]
    pub metadata: Metadata,
}
