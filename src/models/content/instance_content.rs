use super::{super::id::InstanceContentId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceContent {
    #[serde(flatten)]
    pub metadata: Metadata<InstanceContentId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
