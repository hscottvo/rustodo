use crate::error::Result;
use uuid::Uuid;

use crate::core::{item::ItemId, list::ListId};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    NotDone,
    InProgress,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ListEntryId(Uuid);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ListEntry {
    id: ListEntryId,
    item_id: ItemId,
    list_id: ListId,
    status: Status,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CreateListEntryRequest {
    item: ItemId,
    list: ListId,
}
trait ListEntryRepository {
    fn create_list_entry(&self, req: &CreateListEntryRequest) -> Result<ListEntry>;
}
