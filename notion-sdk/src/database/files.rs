use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FileReference {
    pub name: String,
    pub url: String,
    pub mime_type: String,
}
