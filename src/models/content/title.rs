use super::{super::id::TitleId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Title {
    #[serde(flatten)]
    pub metadata: Metadata<TitleId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
