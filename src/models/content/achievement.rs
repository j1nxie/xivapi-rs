use super::{super::id::AchievementId, Metadata};
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Achievement {
    #[serde(flatten)]
    pub metadata: Metadata<AchievementId>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}
