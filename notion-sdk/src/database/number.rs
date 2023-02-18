use serde::{Deserialize, Serialize};

/// How the number is displayed in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    Number,
    NumberWithCommas,
    Percent,
    Dollar,
    Euro,
    Pound,
    Yen,
    Ruble,
    Rupee,
    Won,
    Yuan,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct NumberDetails {
    pub format: NumberFormat,
}
