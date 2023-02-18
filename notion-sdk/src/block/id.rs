use crate::Error;
use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct BlockId(pub uuid::Uuid);

impl Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for BlockId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match uuid::Uuid::parse_str(s) {
            Ok(i) => Ok(BlockId(i)),
            Err(e) => Err(Error::UUID { source: e }),
        }
    }
}
