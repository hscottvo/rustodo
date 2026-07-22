use std::{collections::HashMap, sync::Mutex};

use crate::error::{Error, Result};
use derive_more::From;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ItemId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    id: ItemId,
    content: String,
}

impl Item {
    pub fn new(id: ItemId, content: String) -> Item {
        Item { id, content }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateItemRequest {
    pub(crate) content: String,
}

pub trait ItemRepository {
    fn create_item(&self, req: &CreateItemRequest) -> Result<Item>;
    fn get_item(&self, id: ItemId) -> Result<Item>;
}
