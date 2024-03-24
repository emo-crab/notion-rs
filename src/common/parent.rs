use crate::database::id::DatabaseId;
use crate::pages::id::PageId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Parent {
  DatabaseId {
    database_id: DatabaseId,
  },
  PageId {
    page_id: PageId,
  },
  #[default]
  Workspace,
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use crate::common::parent::Parent;
  use crate::database::id::DatabaseId;

  #[test]
  fn it_works() {
    let p = Parent::DatabaseId { database_id: DatabaseId::from_str("d9824bdc-8445-4327-be8b-5b47500af6ce").unwrap() };
    println!("{:?}", serde_json::to_string_pretty(&p).unwrap());
    let j = r#"{"type": "database_id","database_id": "d9824bdc-8445-4327-be8b-5b47500af6ce"}"#;
    let p: Parent = serde_json::from_str(j).unwrap();
    println!("{:?}", p);
  }
}