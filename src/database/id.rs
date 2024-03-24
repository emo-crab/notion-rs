use crate::error::Error;

use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Default)]
#[serde(transparent)]
pub struct DatabaseId(pub uuid::Uuid);

impl Display for DatabaseId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl std::str::FromStr for DatabaseId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match uuid::Uuid::parse_str(s) {
      Ok(i) => Ok(DatabaseId(i)),
      Err(e) => Err(Error::UUID { source: e }),
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone, Default)]
#[serde(transparent)]
pub struct PropertyId(pub String);

impl Display for PropertyId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl std::str::FromStr for PropertyId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(PropertyId(s.to_string()))
  }
}
