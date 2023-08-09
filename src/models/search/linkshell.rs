use crate::models::id::LinkshellId;
use serde::Deserialize;

use super::Pagination;

use ffxiv_types::World;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
    pub pagination: Pagination,
    pub results: Vec<SearchLinkshell>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchLinkshell {
    #[serde(rename = "ID")]
    pub id: LinkshellId,
    pub name: String,
    pub server: World,
}
