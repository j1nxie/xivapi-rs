// use crate::prelude::Either;
use serde::Deserialize;

use super::{super::id::EnemyId, GamePatch, Metadata};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
    #[serde(flatten)]
    pub metadata: Metadata<EnemyId>,
    // TODO: look at the usage of Either here.
    // pub game_content_links: Either<[!; 0], serde_json::Value>,
    pub game_content_links: serde_json::Value,
    pub game_patch: Option<GamePatch>,
    #[serde(with = "crate::util::serde::int_bool")]
    pub starts_with_vowel: bool,
}
