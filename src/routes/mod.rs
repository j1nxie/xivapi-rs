pub mod character;
pub mod content;
pub mod free_company;
pub mod linkshell;
pub mod search;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "cn")]
    Chinese,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "kr")]
    Korean,
}
