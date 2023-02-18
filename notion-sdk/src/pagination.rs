use crate::block::Block;
use crate::database::properties::PropertyValue;
use crate::database::Database;
use crate::error::ErrorResponse;
use crate::pages::Page;
use crate::user::User;
use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct PagingCursor(String);

#[derive(Serialize, Debug, Eq, PartialEq, Default, Clone)]
pub struct Paging {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<PagingCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,
}

pub trait Pageable {
    fn start_from(self, starting_point: Option<PagingCursor>) -> Self;
}

/// <https://developers.notion.com/reference/pagination#responses>
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub next_cursor: Option<PagingCursor>,
    pub has_more: bool,
}

impl<T> ListResponse<T> {
    pub fn results(&self) -> &[T] {
        &self.results
    }
}
impl ListResponse<Object> {
    pub fn only_databases(self) -> ListResponse<Database> {
        let databases = self
            .results
            .into_iter()
            .filter_map(|object| match object {
                Object::Database { database } => Some(database),
                _ => None,
            })
            .collect();

        ListResponse {
            results: databases,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        }
    }

    pub fn expect_databases(self) -> Result<ListResponse<Database>, Error> {
        let databases: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Database { database } => Ok(database),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: databases?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
    }

    pub fn expect_pages(self) -> Result<ListResponse<Page>, Error> {
        let items: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Page { page } => Ok(page),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: items?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
    }

    pub fn expect_blocks(self) -> Result<ListResponse<Block>, crate::Error> {
        let items: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Block { block } => Ok(block),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: items?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
    }
}

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "object")]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Block {
        #[serde(flatten)]
        block: Block,
    },
    Database {
        #[serde(flatten)]
        database: Database,
    },
    Page {
        #[serde(flatten)]
        page: Page,
    },
    List {
        #[serde(flatten)]
        list: ListResponse<Object>,
    },
    User {
        #[serde(flatten)]
        user: User,
    },
    PropertyItem {
        #[serde(flatten)]
        property_item: PropertyValue,
    },
    Error {
        #[serde(flatten)]
        error: ErrorResponse,
    },
}

impl Object {
    pub fn is_database(&self) -> bool {
        matches!(self, Object::Database { .. })
    }
}
