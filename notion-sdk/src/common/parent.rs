use crate::database::id::DatabaseId;
use crate::pages::id::PageId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Parent {
    #[serde(rename = "database_id")]
    Database {
        database_id: DatabaseId,
    },
    #[serde(rename = "page_id")]
    Page {
        page_id: PageId,
    },
    Workspace,
}
