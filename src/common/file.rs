use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FileObject {
    File { file: InternalFileObject },
    External { external: ExternalFileObject },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct InternalFileObject {
    pub url: String,
    pub expiry_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ExternalFileObject {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FileOrEmojiObject {
    Emoji { emoji: String },
    File { file: InternalFileObject },
    External { external: ExternalFileObject },
}
