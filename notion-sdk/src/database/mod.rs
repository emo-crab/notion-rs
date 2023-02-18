pub mod properties;

use crate::common::file::FileOrEmojiObject;
use crate::common::parent::Parent;
use crate::common::rich_text::RichText;
use crate::database::id::DatabaseId;
use crate::database::properties::{Properties, PropertyConfiguration};
use crate::user::UserCommon;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod aka;
mod api;
pub mod date;
pub mod files;
pub mod formula;
pub mod id;
pub mod number;
pub mod relation;
pub mod rollup;
pub mod select;
pub mod status;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
}

/// Represents a Notion Database
/// See <https://developers.notion.com/reference/database>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Database {
    /// Unique identifier for the database.
    pub id: DatabaseId,
    pub icon: Option<FileOrEmojiObject>,
    /// Date and time when this database was created.
    pub created_time: DateTime<Utc>,
    /// Date and time when this database was updated.
    pub last_edited_time: DateTime<Utc>,
    pub created_by: UserCommon,
    pub last_edited_by: UserCommon,
    /// Name of the database as it appears in Notion.
    pub title: Vec<RichText>,
    /// Schema of properties for the database as they appear in Notion.
    //
    // key string
    // The name of the property as it appears in Notion.
    //
    // value object
    // A Property object.
    pub properties: HashMap<String, PropertyConfiguration>,
    /// The archived status of the database.
    pub archived: bool,
    pub is_inline: bool,
    pub description: Vec<RichText>,
    pub url: String,
    pub parent: Parent,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct CreateDatabase {
    pub parent: Parent,
    pub title: Vec<RichText>,
    pub properties: Properties,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct UpdateDatabase {
    pub title: Vec<RichText>,
    pub properties: Properties,
}
