use crate::error::Result;
use derive_more::From;
use uuid::Uuid;

use crate::core::{item::ItemId, list::ListId};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    NotDone,
    InProgress,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ListEntryId(Uuid);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListEntry {
    id: ListEntryId,
    item_id: ItemId,
    list_id: ListId,
    status: Status,
}

pub trait ListEntryRepository {
    fn create_list_entry(&self, item_id: &ItemId, list_id: &ListId) -> Result<ListEntryId>;
}
