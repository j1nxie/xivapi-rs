use super::{super::id::WeatherId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weather {
    #[serde(flatten)]
    pub metadata: Metadata<WeatherId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
