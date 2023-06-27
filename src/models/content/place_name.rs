use super::{super::id::PlaceNameId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceName {
    #[serde(flatten)]
    pub metadata: Metadata<PlaceNameId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
