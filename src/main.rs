use eyre::{Context, Result};
use rustodo::{
    adapters::json_file::item_repository::JsonFileItemRepository, core::item::ItemRepository,
};

fn main() -> Result<()> {
    let items = JsonFileItemRepository::default();
    items
        .create_item("pour coffee")
        .wrap_err("failed to create item")?;
    items
        .create_item("wash clothes")
        .wrap_err("failed to create item")?;

    items.save()?;

    let load_items = JsonFileItemRepository::load("items.json")?;
    println!("{:?}", items);
    println!("{:?}", load_items);

    Ok(())
}
