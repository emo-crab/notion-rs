use crate::database::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct SelectOptionId(String);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectedValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SelectOptionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub color: Color,
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Select {
    /// Sorted list of options available for this property.
    pub options: Vec<SelectOption>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectOption {
    pub name: String,
    pub id: SelectOptionId,
    pub color: Color,
}
