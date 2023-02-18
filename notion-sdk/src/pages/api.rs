use crate::database::id::PropertyId;
use crate::pages::id::PageId;
use crate::pages::{CreatePage, UpdatePage};
use crate::{Error, NotionApi, Object};

const ROUTER: &str = "pages";

impl NotionApi {
    /// Retrieve a user
    pub async fn pages_retrieve(&self, id: PageId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.get(u)).await
    }
    pub async fn pages_property(
        &self,
        id: PageId,
        property_id: PropertyId,
    ) -> Result<Object, Error> {
        let u = format!(
            "{}/{}/{}/properties/{}",
            self.base_path, ROUTER, id, property_id
        );
        self.request(self.client.get(u)).await
    }
    pub async fn pages_create(&self, page: CreatePage) -> Result<Object, Error> {
        let u = format!("{}/{}/", self.base_path, ROUTER);
        self.request(self.client.post(u).json(&page)).await
    }
    pub async fn pages_update(&self, id: PageId, page: UpdatePage) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.patch(u).json(&page)).await
    }
    pub async fn pages_delete(&self, id: PageId) -> Result<Object, Error> {
        let u = format!("{}/blocks/{}", self.base_path, id);
        self.request(self.client.delete(u)).await
    }
}
