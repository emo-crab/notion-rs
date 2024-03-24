//!
//! # Notion Page
//!
//! ## Examples
//! ```rust,no_run
//! use notion_sdk::common::parent::Parent;
//! use notion_sdk::database::properties::Properties;
//! use notion_sdk::pages::CreatePage;
//! let page = CreatePage{
//!     icon: None,
//!     parent: Parent::Workspace,
//!     properties: Properties { properties: Default::default()},
//!     children: vec![],
//! };
//! ```
mod aka;
mod api;
pub mod id;

use crate::block::Block;
use crate::common::file::FileOrEmojiObject;
use crate::common::parent::Parent;
use crate::database::properties::Properties;
use crate::pages::id::PageId;
use crate::user::UserCommon;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::pagination::Object;

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Builder, Clone)]
#[builder(setter(into), default)]
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

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct CreatePage {
  pub icon: Option<FileOrEmojiObject>,
  pub parent: Parent,
  pub properties: Properties,
  pub children: Vec<Object>,
}

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct UpdatePage {
  pub icon: Option<FileOrEmojiObject>,
  pub properties: Properties,
  pub archived: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct ArchivedPage {
  pub archived: bool,
}