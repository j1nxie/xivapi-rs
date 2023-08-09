use super::{super::id::ActionId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    #[serde(flatten)]
    pub metadata: Metadata<ActionId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
