#[cfg(test)]
mod tests {
  use notion_sdk::NotionApi;
  use notion_sdk::pagination::Object;

  #[tokio::test]
  async fn it_works() {
    let api = NotionApi::new(std::env::var("NOTION_TOKEN").unwrap()).unwrap();
    if let Ok(Object::User { user }) = api.users_me().await {
      println!("{:?}", user);
    };
  }
}
