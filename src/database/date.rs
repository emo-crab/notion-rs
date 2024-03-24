use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DateValue {
    pub start: DateOrDateTime,
    pub end: Option<DateOrDateTime>,
    pub time_zone: Option<String>,
}

/// Formula property value objects represent the result of evaluating a formula
/// described in the database's properties.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FormulaResultValue {
    String { string: Option<String> },
    Number { number: Option<Number> },
    Boolean { boolean: Option<bool> },
    Date { date: Option<DateValue> },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}
