//! Exports cloud-scanner results as json
//!
//!
//!
//!
use crate::impact_provider::CloudResourceWithImpacts;
use serde::{Deserialize, Serialize};

pub struct JSONExporter {}

impl JSONExporter {
    pub fn as_json(vec: &Vec<CloudResourceWithImpacts>) -> String {
        let res = serde_json::to_string(vec).unwrap();
        res
    }
}
