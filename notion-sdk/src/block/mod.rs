mod api;
mod code;
pub mod id;
mod todo;

use crate::block::code::CodeFields;
use crate::block::id::BlockId;
use crate::block::todo::ToDoFields;
use crate::common::file::{FileObject, FileOrEmojiObject};
use crate::common::rich_text::{RichText, Text, TextColor};
use crate::database::id::DatabaseId;
use crate::pages::id::PageId;
use crate::user::UserCommon;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Block {
    Paragraph {
        #[serde(flatten)]
        common: BlockCommon,
        paragraph: TextAndChildren,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        #[serde(flatten)]
        common: BlockCommon,
        heading_1: Text,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        #[serde(flatten)]
        common: BlockCommon,
        heading_2: Text,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        #[serde(flatten)]
        common: BlockCommon,
        heading_3: Text,
    },
    #[serde(rename = "callout")]
    CallOut {
        #[serde(flatten)]
        common: BlockCommon,
        #[serde(rename = "callout")]
        call_out: CallOut,
    },
    Quote {
        #[serde(flatten)]
        common: BlockCommon,
        quote: TextAndChildren,
    },
    BulletedListItem {
        #[serde(flatten)]
        common: BlockCommon,
        bulleted_list_item: TextAndChildren,
    },
    NumberedListItem {
        #[serde(flatten)]
        common: BlockCommon,
        numbered_list_item: TextAndChildren,
    },
    ToDo {
        #[serde(flatten)]
        common: BlockCommon,
        to_do: ToDoFields,
    },
    Toggle {
        #[serde(flatten)]
        common: BlockCommon,
        toggle: TextAndChildren,
    },
    Code {
        #[serde(flatten)]
        common: BlockCommon,
        code: CodeFields,
    },
    ChildPage {
        #[serde(flatten)]
        common: BlockCommon,
        child_page: ChildPageFields,
    },
    ChildDatabase {
        #[serde(flatten)]
        common: BlockCommon,
        child_page: ChildDatabaseFields,
    },
    Embed {
        #[serde(flatten)]
        common: BlockCommon,
        embed: EmbedFields,
    },
    Image {
        #[serde(flatten)]
        common: BlockCommon,
        image: FileObject,
    },
    Video {
        #[serde(flatten)]
        common: BlockCommon,
        video: FileObject,
    },
    File {
        #[serde(flatten)]
        common: BlockCommon,
        file: FileObject,
        caption: Text,
    },
    Pdf {
        #[serde(flatten)]
        common: BlockCommon,
        pdf: FileObject,
    },
    Bookmark {
        #[serde(flatten)]
        common: BlockCommon,
        bookmark: BookmarkFields,
    },
    Equation {
        #[serde(flatten)]
        common: BlockCommon,
        equation: Equation,
    },
    Divider {
        #[serde(flatten)]
        common: BlockCommon,
    },
    TableOfContents {
        #[serde(flatten)]
        common: BlockCommon,
        table_of_contents: TableOfContents,
    },
    Breadcrumb {
        #[serde(flatten)]
        common: BlockCommon,
    },
    ColumnList {
        #[serde(flatten)]
        common: BlockCommon,
        column_list: ColumnListFields,
    },
    Column {
        #[serde(flatten)]
        common: BlockCommon,
        column: ColumnFields,
    },
    LinkPreview {
        #[serde(flatten)]
        common: BlockCommon,
        link_preview: LinkPreviewFields,
    },
    Template {
        #[serde(flatten)]
        common: BlockCommon,
        template: TemplateFields,
    },
    LinkToPage {
        #[serde(flatten)]
        common: BlockCommon,
        link_to_page: LinkToPageFields,
    },
    Table {
        #[serde(flatten)]
        common: BlockCommon,
        table: TableFields,
    },
    SyncedBlock {
        #[serde(flatten)]
        common: BlockCommon,
        synced_block: SyncedBlockFields,
    },
    TableRow {
        #[serde(flatten)]
        common: BlockCommon,
        table_row: TableRowFields,
    },
    Unsupported {
        #[serde(flatten)]
        common: BlockCommon,
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
#[serde(rename_all = "lowercase")]
pub enum CodeLanguage {
    Abap,
    Arduino,
    Bash,
    Basic,
    C,
    Clojure,
    Coffeescript,
    #[serde(rename = "c++")]
    CPlusPlus,
    #[serde(rename = "c#")]
    CSharp,
    Css,
    Dart,
    Diff,
    Docker,
    Elixir,
    Elm,
    Erlang,
    Flow,
    Fortran,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    Glsl,
    Go,
    Graphql,
    Groovy,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    Livescript,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    Ocaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    PlainText,
    Powershell,
    Prolog,
    Protobuf,
    Python,
    R,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Sql,
    Swift,
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    Webassembly,
    Xml,
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaCAndCPlusPlusAndCSharp,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableRowFields {
    pub cells: Vec<RichText>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockCommon {
    pub id: BlockId,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub has_children: bool,
    pub created_by: UserCommon,
    pub last_edited_by: UserCommon,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextAndChildren {
    pub rich_text: Vec<RichText>,
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
