use crate::error::Error;

use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Default)]
#[serde(transparent)]
pub struct PageId(pub uuid::Uuid);

impl Display for PageId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl std::str::FromStr for PageId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match uuid::Uuid::parse_str(s) {
      Ok(i) => Ok(PageId(i)),
      Err(e) => Err(Error::UUID { source: e }),
    }
  }
}
