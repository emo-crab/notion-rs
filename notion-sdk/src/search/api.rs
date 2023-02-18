use crate::search::SearchRequest;
use crate::{Error, NotionApi, Object};

const ROUTER: &str = "search";

impl NotionApi {
    /// Retrieve a user
    pub async fn search(&self, query: SearchRequest) -> Result<Object, Error> {
        let u = format!("{}/{}", self.base_path, ROUTER);
        self.request(self.client.post(u).json(&query)).await
    }
}
