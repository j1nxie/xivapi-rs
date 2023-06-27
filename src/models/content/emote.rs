use super::{super::id::EmoteId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Emote {
    #[serde(flatten)]
    pub metadata: Metadata<EmoteId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
