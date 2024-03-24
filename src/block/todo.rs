use crate::block::Block;
use crate::common::rich_text::{RichText, TextColor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToDoFields {
    pub rich_text: Vec<RichText>,
    pub checked: bool,
    pub children: Option<Vec<Block>>,
    pub color: TextColor,
}
