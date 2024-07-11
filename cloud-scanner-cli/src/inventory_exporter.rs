use anyhow::Context;
use schemars::schema_for;

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

/// Returns the json schema of an inventory as String
pub fn get_inventory_schema() -> anyhow::Result<String> {
    let schema = schema_for!(Inventory);
    let st = serde_json::to_string_pretty(&schema)?;
    Ok(st)
}

/// Print inventory schema on stdout
pub fn print_inventory_schema() -> anyhow::Result<()> {
    let s = get_inventory_schema()?;
    println!("{}", s);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::inventory_exporter::get_inventory_schema;
    const INVENTORY_JSON_SCHEMA: &str = include_str!("../test-data/INVENTORY_JSON_SCHEMA.json");

    #[test]
    pub fn generate_inventory_schema() {
        let s = get_inventory_schema().unwrap();
        println!("{}", s);

        assert_eq!(s, INVENTORY_JSON_SCHEMA, "schema do not match");
    }
}
