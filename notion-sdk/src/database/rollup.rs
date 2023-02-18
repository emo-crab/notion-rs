use crate::common::rich_text::RichText;
use crate::database::date::{DateValue, FormulaResultValue};
use crate::database::files::FileReference;
use crate::database::id::PropertyId;
use crate::database::relation::RelationValue;
use crate::database::select::SelectedValue;
use crate::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RollupValue {
    Number { number: Option<Number> },
    Date { date: Option<DateTime<Utc>> },
    Array { array: Vec<RollupPropertyValue> },
}
/// <https://developers.notion.com/reference/page#rollup-property-value-element>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RollupPropertyValue {
    /// <https://developers.notion.com/reference/page#rich-text-property-values>
    #[serde(rename = "rich_text")]
    Text {
        rich_text: Vec<RichText>,
    },
    /// <https://developers.notion.com/reference/page#number-property-values>
    Number {
        number: Option<Number>,
    },
    /// <https://developers.notion.com/reference/page#select-property-values>
    Select {
        select: Option<SelectedValue>,
    },
    Status {
        status: Option<SelectedValue>,
    },
    MultiSelect {
        multi_select: Option<Vec<SelectedValue>>,
    },
    Date {
        date: Option<DateValue>,
    },
    /// <https://developers.notion.com/reference/page#formula-property-values>
    Formula {
        formula: FormulaResultValue,
    },
    /// <https://developers.notion.com/reference/page#relation-property-values>
    /// It is actually an array of relations
    Relation {
        relation: Option<Vec<RelationValue>>,
    },
    /// <https://developers.notion.com/reference/page#rollup-property-values>
    Rollup {
        rollup: Option<RollupValue>,
    },
    People {
        people: Vec<User>,
    },
    Files {
        files: Option<Vec<FileReference>>,
    },
    Checkbox {
        checkbox: bool,
    },
    Url {
        url: Option<String>,
    },
    Email {
        email: Option<String>,
    },
    PhoneNumber {
        phone_number: String,
    },
    CreatedTime {
        created_time: DateTime<Utc>,
    },
    CreatedBy {
        created_by: User,
    },
    LastEditedTime {
        last_edited_time: DateTime<Utc>,
    },
    LastEditedBy {
        last_edited_by: User,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Rollup {
    /// The name of the relation property this property is responsible for rolling up.
    pub relation_property_name: String,
    /// The id of the relation property this property is responsible for rolling up.
    pub relation_property_id: PropertyId,
    /// The name of the property of the pages in the related database
    /// that is used as an input to `function`.
    pub rollup_property_name: String,
    /// The id of the property of the pages in the related database
    /// that is used as an input to `function`.
    pub rollup_property_id: String,
    /// The function that is evaluated for every page in the relation of the rollup.
    pub function: RollupFunction,
}

/// The function used to roll up the values of the relation property.
/// <https://developers.notion.com/reference/page-property-values#rollup>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Average,
    Checked,
    Count,
    CountPerGroup,
    CountValues,
    DateRange,
    EarliestDate,
    Empty,
    LatestDate,
    Max,
    Median,
    Min,
    NotEmpty,
    PercentChecked,
    PercentEmpty,
    PercentNotEmpty,
    PercentPerGroup,
    PercentUnchecked,
    Range,
    ShowOriginal,
    ShowUnique,
    Sum,
    Unchecked,
    Unique,
}
