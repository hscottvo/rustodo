use crate::{
    core::list_entry::{self, ListEntry, ListEntryId, ListEntryRepository},
    error::Result,
};
use uuid::Uuid;

use crate::core::item::ItemId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListId(Uuid);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    id: ListId,
    entries: Vec<ListEntryId>,
}

impl List {
    pub fn add_entry_to_list(&mut self, entries: &[ListEntryId]) {
        self.entries.extend_from_slice(entries);
    }
}

pub trait ListRepository {
    fn create_list(&self, req: &[ItemId]) -> Result<List>;
    fn append_to_list(&self, entries: &[ItemId]) -> Result<List>;
}
