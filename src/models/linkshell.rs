use super::{id::LinkshellId, search::Pagination, Member};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkshellResult {
    pub linkshell: Option<Linkshell>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Linkshell {
    #[serde(rename = "ID")]
    pub id: LinkshellId,
    pub pagination: Pagination,
    pub profile: LinkshellProfile,
    #[serde(rename = "Results")]
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkshellProfile {
    pub name: String,
    pub server: String,
}
