mod aka;
mod api;
mod builder;
pub mod id;

use crate::block::Block;
use crate::common::file::FileOrEmojiObject;
use crate::common::parent::Parent;
use crate::database::properties::Properties;
use crate::pages::id::PageId;
use crate::user::UserCommon;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub id: PageId,
    pub icon: Option<FileOrEmojiObject>,
    /// Date and time when this page was created.
    pub created_time: DateTime<Utc>,
    /// User who created the page.
    pub created_by: UserCommon,
    /// Date and time when this page was updated.
    pub last_edited_time: DateTime<Utc>,
    /// User who last edited the page.
    pub last_edited_by: UserCommon,
    /// The archived status of the page.
    pub archived: bool,
    pub properties: Properties,
    pub parent: Parent,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct CreatePage {
    pub icon: Option<FileOrEmojiObject>,
    pub parent: Parent,
    pub properties: Properties,
    pub children: Vec<Block>,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct UpdatePage {
    pub icon: Option<FileOrEmojiObject>,
    pub properties: Properties,
    pub archived: bool,
}
