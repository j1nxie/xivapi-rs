pub use crate::{builder::Builder, XivApi};

use serde::{Deserialize, Serialize};

pub use ffxiv_types::World;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
