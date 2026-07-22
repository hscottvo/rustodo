use crate::error::Result;
use uuid::Uuid;

use crate::core::item::ItemId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListId(Uuid);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    id: ListId,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateListRequest {
    entries: Vec<ItemId>,
}
trait ListRepository {
    fn create_list(&self, req: &CreateListRequest) -> Result<List>;
}
