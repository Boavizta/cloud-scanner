//! A module to allow retrieving impacts of cloud resources
//!  It define s a Impact procvider  trait that you should use when implementing vendor specific inventory .
///
/// The model of impacts goes here (scan result summary ?)
///
/// The model of allocation should be internal to boa API
///
use crate::cloud_resource::*;
use anyhow::{Context, Result};
use async_trait::async_trait;

/// A ImpactProvider trait that yu should implement for a specific impact API
///
/// Implementing this trait when creating a new ImpactProvider (for example to support a different version of Boavizata db) ensures that cloud-scanner will be able to use it.
#[async_trait]
pub trait ImpactProvider {
    /// Returns a list list of CloudImpacts
    async fn get_impacts(
        &self,
        resources: Vec<CloudResource>,
    ) -> Result<Vec<CloudResourceWithImpacts>>;
}

#[derive(Clone, Debug)]
pub struct CloudResourceWithImpacts {
    pub cloud_resource: CloudResource,
    pub resource_impacts: Option<ResourceImpacts>,
}

/// Impacts of an individual resource
#[derive(Clone, Debug, Default)]
pub struct ResourceImpacts {
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
}

/// The aggregated impacts and meta data about the scan results
#[derive(Clone, Debug, Default)]
pub struct ImpactsSummary {
    pub number_of_instances_total: u32,
    pub number_of_instances_assessed: u32,
    pub number_of_instances_not_assessed: u32,
    pub duration_of_use_hours: f64,
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
    pub aws_region: String,
    pub country: String,
}

impl ImpactsSummary {
    pub fn new(
        aws_region: String,
        country: String,
        resources: Vec<CloudResourceWithImpacts>,
    ) -> Self {
        //     if (resources.len())
        // let first_instance =

        let mut summary = ImpactsSummary {
            number_of_instances_total: u32::try_from(resources.len()).unwrap(),
            aws_region,
            country,
            ..Default::default()
        };

        for resource in resources {
            // Only consider the instances for which we have impact data
            if let Some(impacts) = resource.resource_impacts {
                summary.number_of_instances_assessed += 1;
                summary.adp_manufacture_kgsbeq += impacts.adp_manufacture_kgsbeq;
                summary.adp_use_kgsbeq += impacts.adp_use_kgsbeq;
                summary.pe_manufacture_megajoules += impacts.pe_manufacture_megajoules;
                summary.pe_use_megajoules += impacts.pe_use_megajoules;
                summary.gwp_manufacture_kgco2eq += impacts.gwp_manufacture_kgco2eq;
                summary.gwp_use_kgco2eq += impacts.gwp_use_kgco2eq;
            } else {
                // Resource was not counted due to no impact
                debug!("Skipped counting resource: {:#?} while building summary because it has no impact data", resource);
                summary.number_of_instances_not_assessed += 1;
            }
        }
        summary
    }
}

/*
if let Some(results) = raw_result {
       debug!("This cloud resource has impacts data: {}", results);
       resource_impacts = ResourceImpacts {
           adp_manufacture_kgsbeq: results["adp"]["manufacture"].as_f64().unwrap(),
           adp_use_kgsbeq: results["adp"]["use"].as_f64().unwrap(),
           pe_manufacture_megajoules: results["pe"]["manufacture"].as_f64().unwrap(),
           pe_use_megajoules: results["pe"]["use"].as_f64().unwrap(),
           gwp_manufacture_kgco2eq: results["gwp"]["manufacture"].as_f64().unwrap(),
           gwp_use_kgco2eq: results["gwp"]["use"].as_f64().unwrap(),
       };
*/
/*
/// Returns a summary (summing/aggregating data where possible) of the scan results.
pub async fn build_summary(
    instances_with_impacts: &Vec<AwsInstanceWithImpacts>,
    aws_region: &str,
    duration_of_use_hours: f64,
) -> Result<ScanResultSummary> {
    let number_of_instances_total = u32::try_from(instances_with_impacts.len())?;

    let usage_location: UsageLocation = UsageLocation::from(aws_region);

    let mut summary = ScanResultSummary {
        number_of_instances_total,
        aws_region: aws_region.to_owned(),
        country: usage_location.iso_country_code,
        duration_of_use_hours,
        ..Default::default()
    };

    for instance in instances_with_impacts {
        // Only consider the instances for which we have impact data
        if let Some(impacts) = &instance.impacts {
            debug!("This instance has impacts data: {}", impacts);
            summary.number_of_instances_assessed += 1;
            summary.adp_manufacture_kgsbeq += impacts["adp"]["manufacture"].as_f64().unwrap();
            summary.adp_use_kgsbeq += impacts["adp"]["use"].as_f64().unwrap();
            summary.pe_manufacture_megajoules += impacts["pe"]["manufacture"].as_f64().unwrap();
            summary.pe_use_megajoules += impacts["pe"]["use"].as_f64().unwrap();
            summary.gwp_manufacture_kgco2eq += impacts["gwp"]["manufacture"].as_f64().unwrap();
            summary.gwp_use_kgco2eq += impacts["gwp"]["use"].as_f64().unwrap();
        } else {
            debug!("Skipped instance: {:#?} while building summary because instance has no impact data", instance);
        }
    }

    summary.number_of_instances_not_assessed =
        summary.number_of_instances_total - summary.number_of_instances_assessed;

    Ok(summary)
}
*/
