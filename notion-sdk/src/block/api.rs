use crate::block::id::BlockId;
use crate::block::UpdateBlock;
use crate::pages::id::PageId;
use crate::{Error, NotionApi, Object};

const ROUTER: &str = "blocks";

impl NotionApi {
    /// Retrieve a user
    pub async fn block_retrieve(&self, id: BlockId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.get(u)).await
    }
    pub async fn block_children(&self, id: BlockId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}/children", self.base_path, ROUTER, id);
        self.request(self.client.get(u)).await
    }
    pub async fn block_append_children(&self, id: PageId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}/children", self.base_path, ROUTER, id);
        self.request(self.client.patch(u)).await
    }
    pub async fn block_delete(&self, id: BlockId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.delete(u)).await
    }
    pub async fn block_update(&self, id: BlockId, db: UpdateBlock) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.patch(u).json(&db)).await
    }
}
