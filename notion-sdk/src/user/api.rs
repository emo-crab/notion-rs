use crate::user::id::UserId;
use crate::{Error, NotionApi, Object};

const ROUTER: &str = "users";

impl NotionApi {
    /// list users
    pub async fn users_list(&self) -> Result<Object, Error> {
        let u = format!("{}/{}", self.base_path, ROUTER);
        self.request(self.client.get(u)).await
    }
    /// Retrieve a user
    pub async fn users_retrieve(&self, id: UserId) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, id);
        self.request(self.client.get(u)).await
    }
    /// Retrieve bot's user info
    pub async fn users_me(&self) -> Result<Object, Error> {
        let u = format!("{}/{}/{}", self.base_path, ROUTER, "me");
        self.request(self.client.get(u)).await
    }
}
