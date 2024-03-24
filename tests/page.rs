#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use notion_sdk::block::{Block, BlockText, TextAndChildren};
  use notion_sdk::common::parent::Parent;
  use notion_sdk::common::rich_text::{RichText, RichTextCommon, Text};
  use notion_sdk::database::id::DatabaseId;
  use notion_sdk::database::properties::{Properties, PropertyValue};
  use notion_sdk::NotionApi;
  use notion_sdk::pages::{CreatePageBuilder};
  use notion_sdk::pagination::Object;

  #[tokio::test]
  async fn it_works() {
    let api = NotionApi::new(std::env::var("NOTION_TOKEN").unwrap()).unwrap();
    let properties = Properties::default().set("Name", PropertyValue::Title {
      id: Default::default(),
      title: vec![
        RichText::Text {
          common: Default::default(),
          text: Text { content: "Title".to_string(), link: None },
        },
      ],
    },
    );
    let mut blocks = Vec::new();
    blocks.push(Object::Block {
      block: Block::Heading2 {
        common: Default::default(),
        heading_2: BlockText {
          rich_text: vec![
            RichText::Text { common: Default::default(), text: Text { content: "FF".to_string(), link: None } }
          ]
        },
        is_toggleable: false,
      },
    });
    let page = CreatePageBuilder::default().properties(properties).parent(Parent::DatabaseId {
      database_id: DatabaseId::from_str("0398135af459422592f6c02e5a5bfb2d").unwrap(),
    }).children(blocks).build().unwrap();
    println!("{}", serde_json::to_string_pretty(&page).unwrap());
    let result = api.pages_create(page).await.unwrap();
    println!("{:?}", result);
  }
}
