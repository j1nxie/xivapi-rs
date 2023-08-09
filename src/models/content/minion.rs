use super::{super::id::MinionId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Minion {
    #[serde(flatten)]
    pub metadata: Metadata<MinionId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
