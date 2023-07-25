use std::time::Duration;

use serde::{Deserialize, Serialize};
use std::fmt;

///  Statistics about program execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionStatistics {
    pub inventory_duration: Duration,
    pub impact_duration: Duration,
    pub total_duration: Duration,
}

impl fmt::Display for ExecutionStatistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
