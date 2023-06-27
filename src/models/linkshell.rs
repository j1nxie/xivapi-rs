use super::{search::character::SearchCharacter, LodestoneInfo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkshellResult {
    pub info: Info,
    pub linkshell: Vec<SearchCharacter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    pub linkshell: LodestoneInfo,
}
