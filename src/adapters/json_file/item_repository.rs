use std::{collections::HashMap, fs, path::PathBuf, sync::Mutex};

use uuid::Uuid;

use crate::{
    core::item::{Item, ItemId, ItemRepository},
    error::{Error, Result},
};

#[derive(Debug)]
pub struct JsonFileItemRepository {
    table: Mutex<HashMap<ItemId, String>>,
    path: PathBuf,
}

impl JsonFileItemRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let table = Mutex::new(HashMap::new());
        JsonFileItemRepository {
            table,
            path: path.into(),
        }
    }

    pub fn load(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let contents = fs::read_to_string(path.clone())?;
        let table = Mutex::new(serde_json::from_str(&contents)?);
        Ok(JsonFileItemRepository { table, path })
    }

    pub fn save(&self) -> Result<()> {
        let lock = self.table.lock().or(Err(Error::Lock))?;
        let json = serde_json::to_string_pretty(&*lock)?;

        fs::write(self.path.clone(), json)?;

        Ok(())
    }
}

impl ItemRepository for JsonFileItemRepository {
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

impl Default for JsonFileItemRepository {
    fn default() -> Self {
        Self::new("./items.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_save_roundtrip() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("items.json");
        let save_repo = JsonFileItemRepository::new(path.clone());
        save_repo.create_item("a")?;
        save_repo.create_item("b")?;
        save_repo.save()?;

        let load_repo = JsonFileItemRepository::load(path)?;
        assert_eq!(
            &*save_repo.table.lock().or(Err(Error::Lock))?,
            &*load_repo.table.lock().or(Err(Error::Lock))?
        );

        Ok(())
    }

    #[test]
    fn load_missing_file() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("items.json");
        let repo = JsonFileItemRepository::load(path);

        assert!(matches!(repo, Err(Error::Io(_))));

        Ok(())
    }

    #[test]
    fn load_path_is_directory() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let repo = JsonFileItemRepository::load(dir.path());

        assert!(matches!(repo, Err(Error::Io(_))));

        Ok(())
    }

    #[test]
    fn load_malformed_file() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("items.json");

        fs::write(&path, "not json")?;

        let repo = JsonFileItemRepository::load(path);

        assert!(matches!(repo, Err(Error::Serialize(_))));

        Ok(())
    }
}
