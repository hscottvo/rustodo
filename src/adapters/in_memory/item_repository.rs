#[cfg(test)]
use std::{collections::HashMap, sync::Mutex};

#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::{
    core::item::{Item, ItemId, ItemRepository},
    error::{Error, Result},
};

#[cfg(test)]
struct InMemoryItemRepository {
    table: Mutex<HashMap<ItemId, String>>,
}

#[cfg(test)]
impl ItemRepository for InMemoryItemRepository {
    fn create_item(&self, req: impl Into<String>) -> Result<Item> {
        let id = ItemId::from(Uuid::new_v4());

        let content: String = req.into();
        let item = Item::new(id.clone(), content.clone());

        let mut lock = self.table.lock().or(Err(Error::Lock))?;
        lock.insert(id, content);

        Ok(item)
    }

    fn get_item(&self, id: ItemId) -> Result<Item> {
        let lock = self.table.lock().or(Err(Error::Lock))?;
        let content = lock.get(&id).ok_or(Error::ItemDoesNotExist)?.to_owned();
        Ok(Item::new(id, content))
    }
}
