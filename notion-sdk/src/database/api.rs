use crate::database::id::DatabaseId;
use crate::database::{CreateDatabase, UpdateDatabase};
use crate::search::DatabaseQuery;
use crate::{Error, NotionApi, Object};

const ROUTER: &str = "databases";

impl NotionApi {
    /// Retrieve a user
    pub async fn databases_retrieve(&self, id: DatabaseId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.get(u)).await
    }
    pub async fn databases_query(
        &self,
        id: DatabaseId,
        query: Option<DatabaseQuery>,
    ) -> Result<Object, Error> {
        let u = format!("{}/{}/{}/query", self.base_path, ROUTER, id);
        let mut rb = self.client.post(u);
        if let Some(q) = query {
            rb = rb.json(&q);
        }
        self.request(rb).await
    }
    pub async fn databases_create(&self, db: CreateDatabase) -> Result<Object, Error> {
        let u = format!("{}/{}/", self.base_path, ROUTER);
        self.request(self.client.post(u).json(&db)).await
    }
    pub async fn databases_update(
        &self,
        id: DatabaseId,
        db: UpdateDatabase,
    ) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.patch(u).json(&db)).await
    }
}
