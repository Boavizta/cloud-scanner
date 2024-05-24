use anyhow::Context;

use crate::model::Inventory;

/// Returns the inventory of cloud resources as json String
pub async fn get_inventory_as_json(inventory: &Inventory) -> anyhow::Result<String> {
    serde_json::to_string(&inventory).context("Cannot format inventory as json")
}

/// Print inventory to stdout
pub async fn print_inventory(inventory: &Inventory) -> anyhow::Result<()> {
    let json_inventory: String = get_inventory_as_json(inventory).await?;
    println!("{}", json_inventory);
    Ok(())
}
