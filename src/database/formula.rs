use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Formula {
    /// Formula to evaluate for this property
    pub expression: String,
}
