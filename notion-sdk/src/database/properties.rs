use crate::common::rich_text::RichText;
use crate::database::date::{DateValue, FormulaResultValue};
use crate::database::files::FileReference;
use crate::database::formula::Formula;
use crate::database::id::PropertyId;
use crate::database::number::NumberDetails;
use crate::database::relation::{Relation, RelationValue};
use crate::database::rollup::{Rollup, RollupValue};
use crate::database::select::{Select, SelectedValue};
use crate::database::status::Status;
use crate::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Properties {
    #[serde(flatten)]
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PropertyValue {
    // <https://developers.notion.com/reference/property-object#title-configuration>
    Title {
        id: PropertyId,
        title: Vec<RichText>,
    },
    /// <https://developers.notion.com/reference/property-object#text-configuration>
    #[serde(rename = "rich_text")]
    Text {
        id: PropertyId,
        rich_text: Vec<RichText>,
    },
    /// <https://developers.notion.com/reference/property-object#number-configuration>
    Number {
        id: PropertyId,
        number: Option<Number>,
    },
    /// <https://developers.notion.com/reference/property-object#select-configuration>
    Select {
        id: PropertyId,
        select: Option<SelectedValue>,
    },
    /// <https://developers.notion.com/reference/property-object#status-configuration>
    Status {
        id: PropertyId,
        status: Option<SelectedValue>,
    },
    /// <https://developers.notion.com/reference/property-object#multi-select-configuration>
    MultiSelect {
        id: PropertyId,
        multi_select: Option<Vec<SelectedValue>>,
    },
    /// <https://developers.notion.com/reference/property-object#date-configuration>
    Date {
        id: PropertyId,
        date: Option<DateValue>,
    },
    /// <https://developers.notion.com/reference/property-object#formula-configuration>
    Formula {
        id: PropertyId,
        formula: FormulaResultValue,
    },
    /// <https://developers.notion.com/reference/property-object#relation-configuration>
    /// It is actually an array of relations
    Relation {
        id: PropertyId,
        relation: Option<Vec<RelationValue>>,
    },
    /// <https://developers.notion.com/reference/property-object#rollup-configuration>
    Rollup {
        id: PropertyId,
        rollup: Option<RollupValue>,
    },
    /// <https://developers.notion.com/reference/property-object#people-configuration>
    People { id: PropertyId, people: Vec<User> },
    /// <https://developers.notion.com/reference/property-object#files-configuration>
    Files {
        id: PropertyId,
        files: Option<Vec<FileReference>>,
    },
    /// <https://developers.notion.com/reference/property-object#checkbox-configuration>
    Checkbox { id: PropertyId, checkbox: bool },
    /// <https://developers.notion.com/reference/property-object#url-configuration>
    Url { id: PropertyId, url: Option<String> },
    /// <https://developers.notion.com/reference/property-object#email-configuration>
    Email {
        id: PropertyId,
        email: Option<String>,
    },
    /// <https://developers.notion.com/reference/property-object#phone-number-configuration>
    PhoneNumber {
        id: PropertyId,
        phone_number: String,
    },
    /// <https://developers.notion.com/reference/property-object#created-time-configuration>
    CreatedTime {
        id: PropertyId,
        created_time: DateTime<Utc>,
    },
    /// <https://developers.notion.com/reference/property-object#created-by-configuration>
    CreatedBy { id: PropertyId, created_by: User },
    /// <https://developers.notion.com/reference/property-object#last-edited-time-configuration>
    LastEditedTime {
        id: PropertyId,
        last_edited_time: DateTime<Utc>,
    },
    /// <https://developers.notion.com/reference/property-object#last-edited-by-configuration>
    LastEditedBy {
        id: PropertyId,
        last_edited_by: User,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PropertyConfiguration {
    /// Represents the special Title property required on every database.
    /// See <https://developers.notion.com/reference/database#title-configuration>
    Title { id: PropertyId, name: String },
    /// Represents a Text property
    /// <https://developers.notion.com/reference/database#text-configuration>
    #[serde(rename = "rich_text")]
    Text { id: PropertyId, name: String },
    /// Represents a Number Property
    /// See <https://developers.notion.com/reference/database#number-configuration>
    Number {
        id: PropertyId,
        name: String,
        /// How the number is displayed in Notion.
        number: NumberDetails,
    },
    /// Represents a Select Property
    /// See <https://developers.notion.com/reference/database#select-configuration>
    Select {
        id: PropertyId,
        name: String,
        select: Select,
    },
    /// Represents a Status property
    Status {
        id: PropertyId,
        name: String,
        status: Status,
    },
    /// Represents a Multi-select Property
    /// See <https://developers.notion.com/reference/database#multi-select-configuration>
    MultiSelect {
        id: PropertyId,
        name: String,
        multi_select: Select,
    },
    /// Represents a Date Property
    /// See <https://developers.notion.com/reference/database#date-configuration>
    Date { id: PropertyId, name: String },
    /// Represents a People Property
    /// See <https://developers.notion.com/reference/database#people-configuration>
    People { id: PropertyId, name: String },
    /// Represents a File Property
    /// See <https://developers.notion.com/reference/database#file-configuration>
    // Todo: File a bug with notion
    //       Documentation issue: docs claim type name is `file` but it is in fact `files`
    Files { id: PropertyId, name: String },
    /// Represents a Checkbox Property
    /// See <https://developers.notion.com/reference/database#checkbox-configuration>
    Checkbox { id: PropertyId, name: String },
    /// Represents a URL Property
    /// See <https://developers.notion.com/reference/database#url-configuration>
    Url { id: PropertyId, name: String },
    /// Represents a Email Property
    /// See <https://developers.notion.com/reference/database#email-configuration>
    Email { id: PropertyId, name: String },
    /// Represents a Phone number Property
    /// See <https://developers.notion.com/reference/database#phone-number-configuration>
    PhoneNumber { id: PropertyId, name: String },
    /// See <https://developers.notion.com/reference/database#formula-configuration>
    Formula {
        id: PropertyId,
        name: String,
        formula: Formula,
    },
    /// See <https://developers.notion.com/reference/database#relation-configuration>
    Relation {
        id: PropertyId,
        name: String,
        relation: Relation,
    },
    /// See <https://developers.notion.com/reference/database#rollup-configuration>
    Rollup {
        id: PropertyId,
        name: String,
        rollup: Rollup,
    },
    /// See <https://developers.notion.com/reference/database#created-time-configuration>
    CreatedTime { id: PropertyId, name: String },
    /// See <https://developers.notion.com/reference/database#created-by-configuration>
    CreatedBy { id: PropertyId, name: String },
    /// See <https://developers.notion.com/reference/database#last-edited-time-configuration>
    LastEditedTime { id: PropertyId, name: String },
    /// See <https://developers.notion.com/reference/database#last-edited-by-configuration>
    LastEditBy { id: PropertyId, name: String },
}
