use super::{super::id::ItemId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(flatten)]
    pub metadata: Metadata<ItemId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
