use crate::database::date::DateValue;
use crate::database::Database;
use crate::pages::Page;
use crate::user::User;
use serde::{Deserialize, Serialize};

/// Rich text objects contain data for displaying formatted text, mentions, and equations.
/// A rich text object also contains annotations for style information.
/// Arrays of rich text objects are used within property objects and property
/// value objects to create what a user sees as a single text value in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichText {
    /// See <https://developers.notion.com/reference/rich-text#text-objects>
    Text {
        #[serde(flatten)]
        rich_text: RichTextCommon,
        text: Text,
    },
    /// See <https://developers.notion.com/reference/rich-text#mention-objects>
    Mention {
        #[serde(flatten)]
        rich_text: RichTextCommon,
        mention: MentionObject,
    },
    /// See <https://developers.notion.com/reference/rich-text#equation-objects>
    Equation {
        #[serde(flatten)]
        rich_text: RichTextCommon,
    },
}

impl RichText {
    pub fn plain_text(&self) -> &str {
        use RichText::*;
        match self {
            Text { rich_text, .. } | Mention { rich_text, .. } | Equation { rich_text, .. } => {
                &rich_text.plain_text
            }
        }
    }
}

/// Properties common on all rich text objects
/// See <https://developers.notion.com/reference/rich-text#all-rich-text>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RichTextCommon {
    pub plain_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

/// Rich text annotations
/// See <https://developers.notion.com/reference/rich-text#annotations>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Annotations {
    pub bold: Option<bool>,
    pub code: Option<bool>,
    pub color: Option<TextColor>,
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Text {
    pub content: String,
    pub link: Option<Link>,
}

/// See <https://developers.notion.com/reference/rich-text#mention-objects>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MentionObject {
    User {
        user: User,
    },
    Page {
        page: Page,
    },
    Database {
        database: Database,
    },
    Date {
        date: DateValue,
    },
    // LinkPreview {
    //
    // },
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
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
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Link {
    pub url: String,
}
