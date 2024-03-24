//!
//! # Notion Block
//!
//! ## Examples
//! ```rust,no_run
//! use notion_sdk::block::TextAndChildren;
//! use notion_sdk::common::rich_text::TextColor;
//! let text = TextAndChildren{
//!     rich_text: vec![],
//!     children: None,
//!     color: TextColor::Default
//! };
//! ```
mod api;
pub mod code;
pub mod id;
pub mod todo;

use crate::block::code::CodeFields;
use crate::block::id::BlockId;
use crate::block::todo::ToDoFields;
use crate::common::file::{FileObject, FileOrEmojiObject};
use crate::common::rich_text::{RichText, TextColor};
use crate::database::id::DatabaseId;
use crate::pages::id::PageId;
use crate::user::UserCommon;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::common::parent::Parent;
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockText {
  pub rich_text: Vec<RichText>,
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Block {
  Paragraph {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    paragraph: TextAndChildren,
  },
  #[serde(rename = "heading_1")]
  Heading1 {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    heading_1: BlockText,
    is_toggleable: bool,
  },
  #[serde(rename = "heading_2")]
  Heading2 {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    heading_2: BlockText,
    is_toggleable: bool,
  },
  #[serde(rename = "heading_3")]
  Heading3 {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    heading_3: BlockText,
    is_toggleable: bool,
  },
  #[serde(rename = "callout")]
  CallOut {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    #[serde(rename = "callout")]
    call_out: CallOut,
  },
  Quote {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    quote: TextAndChildren,
  },
  BulletedListItem {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    bulleted_list_item: TextAndChildren,
  },
  NumberedListItem {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    numbered_list_item: TextAndChildren,
  },
  ToDo {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    to_do: ToDoFields,
  },
  Toggle {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    toggle: TextAndChildren,
  },
  Code {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    code: CodeFields,
  },
  ChildPage {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    child_page: ChildPageFields,
  },
  ChildDatabase {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    child_page: ChildDatabaseFields,
  },
  Embed {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    embed: EmbedFields,
  },
  Image {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    image: FileObject,
  },
  Video {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    video: FileObject,
  },
  File {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    file: FileObject,
    caption: BlockText,
  },
  Pdf {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    pdf: FileObject,
  },
  Bookmark {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    bookmark: BookmarkFields,
  },
  Equation {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    equation: Equation,
  },
  Divider {
    #[serde(flatten)]
    common: Option<BlockCommon>,
  },
  TableOfContents {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    table_of_contents: TableOfContents,
  },
  Breadcrumb {
    #[serde(flatten)]
    common: Option<BlockCommon>,
  },
  ColumnList {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    column_list: ColumnListFields,
  },
  Column {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    column: ColumnFields,
  },
  LinkPreview {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    link_preview: LinkPreviewFields,
  },
  Template {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    template: TemplateFields,
  },
  LinkToPage {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    link_to_page: LinkToPageFields,
  },
  Table {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    table: TableFields,
  },
  SyncedBlock {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    synced_block: SyncedBlockFields,
  },
  TableRow {
    #[serde(flatten)]
    common: Option<BlockCommon>,
    table_row: TableRowFields,
  },
  Unsupported {
    #[serde(flatten)]
    common: Option<BlockCommon>,
  },
  #[serde(other)]
  Unknown,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPageFields {
  pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabaseFields {
  pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EmbedFields {
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BookmarkFields {
  pub url: String,
  pub caption: Vec<RichText>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Equation {
  pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableOfContents {
  pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnListFields {
  pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnFields {
  pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewFields {
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TemplateFields {
  pub rich_text: Vec<RichText>,
  pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum LinkToPageFields {
  PageId { page_id: PageId },
  DatabaseId { database_id: DatabaseId },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedFromObject {
  pub block_id: BlockId,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedBlockFields {
  pub synced_from: Option<SyncedFromObject>,
  pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableFields {
  pub table_width: u64,
  pub has_column_header: bool,
  pub has_row_header: bool,
  pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct TableRowFields {
  pub cells: Vec<RichText>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockCommon {
  #[serde(default = "default_block")]
  pub object: String,
  pub parent: Parent,
  pub id: BlockId,
  pub created_time: DateTime<Utc>,
  pub last_edited_time: DateTime<Utc>,
  pub has_children: bool,
  pub archived: bool,
  pub created_by: UserCommon,
  pub last_edited_by: UserCommon,
}

impl Default for BlockCommon {
  fn default() -> Self {
    Self {
      object: default_block(),
      parent: Default::default(),
      id: Default::default(),
      created_time: Default::default(),
      last_edited_time: Default::default(),
      has_children: false,
      archived: false,
      created_by: Default::default(),
      last_edited_by: Default::default(),
    }
  }
}

fn default_block() -> String {
  String::from("block")
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextAndChildren {
  pub rich_text: Vec<RichText>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub children: Option<Vec<Block>>,
  pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CallOut {
  pub rich_text: Vec<RichText>,
  pub icon: FileOrEmojiObject,
  pub color: TextColor,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct UpdateBlock {
  pub paragraph: Option<TextAndChildren>,
  pub archived: bool,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct CreateBlock {
  pub archived: bool,
  pub children: Vec<Block>,
}
