use super::{super::id::StatusId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    #[serde(flatten)]
    pub metadata: Metadata<StatusId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
