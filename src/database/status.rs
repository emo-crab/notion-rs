use crate::database::select::{SelectOption, SelectOptionId};
use crate::database::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct StatusGroupOption {
    pub name: String,
    pub id: SelectOptionId,
    pub color: Color,
    pub option_ids: Vec<SelectOptionId>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Status {
    /// Sorted list of options available for this property.
    pub options: Vec<SelectOption>,
    /// Sorted list of groups available for this property.
    pub groups: Vec<StatusGroupOption>,
}
