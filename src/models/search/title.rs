use super::Metadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Title {
    #[serde(flatten)]
    pub metadata: Metadata,
    pub name_female: String,
}
