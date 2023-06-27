use super::{super::id::LeveId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Leve {
    #[serde(flatten)]
    pub metadata: Metadata<LeveId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
