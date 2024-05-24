use anyhow::Result;

use crate::impact_provider::ImpactsSummary;
use crate::model::EstimatedInventory;
use crate::usage_location::UsageLocation;

/// Build a summary (aggregated data) from an estimated inventory
pub async fn build_impact_summary(
    estimated_inventory: &EstimatedInventory,
    aws_region: &str,
    use_duration_hours: &f32,
) -> Result<ImpactsSummary> {
    let usage_location: UsageLocation = UsageLocation::try_from(aws_region)?;
    let summary: ImpactsSummary = ImpactsSummary::new(
        String::from(aws_region),
        usage_location.iso_country_code,
        estimated_inventory,
        (*use_duration_hours).into(),
    );
    debug!("Summary: {:#?}", summary);
    Ok(summary)
}

/// Convert an estimated inventory (inventory with impacts) into json
pub async fn get_estimated_inventory_as_json(
    estimated_inventory: &EstimatedInventory,
    aws_region: &str,
    use_duration_hours: &f32,
    summary_only: bool,
) -> Result<String> {
    if summary_only {
        let summary: ImpactsSummary =
            build_impact_summary(estimated_inventory, aws_region, use_duration_hours).await?;
        return Ok(serde_json::to_string(&summary)?);
    }
    Ok(serde_json::to_string(&estimated_inventory)?)
}
