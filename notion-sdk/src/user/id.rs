use crate::error::Error;

use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct UserId(pub uuid::Uuid);
impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for UserId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match uuid::Uuid::parse_str(s) {
            Ok(i) => Ok(UserId(i)),
            Err(e) => Err(Error::UUID { source: e }),
        }
    }
}
