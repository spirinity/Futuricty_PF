use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Macro untuk auto-generate common derive traits (Debug, Deserialize, Serialize, Clone)
/// Pure functional: eliminates repetitive derive decorators
#[macro_export]
macro_rules! serde_clone {
    ($($body:tt)*) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        $($body)*
    };
}

/// Macro untuk struct yang perlu Default trait juga
#[macro_export]
macro_rules! serde_clone_default {
    ($($body:tt)*) => {
        #[derive(Debug, Deserialize, Serialize, Clone, Default)]
        $($body)*
    };
}

/// Macro untuk struct yang tidak perlu Clone
#[macro_export]
macro_rules! serde_only {
    ($($body:tt)*) => {
        #[derive(Debug, Deserialize, Serialize)]
        $($body)*
    };
}

serde_clone!(
pub struct LocationData {
    pub address: String,
    pub facility_counts: FacilityCounts,
    pub scores: Scores,
    pub nearby_facilities: Vec<String>,
    pub facilities: Vec<Facility>,
}
);

serde_clone_default!(
pub struct FacilityCounts {
    pub health: usize,
    pub education: usize,
    pub market: usize,
    pub transport: usize,
    pub walkability: usize,
    pub recreation: usize,
    pub safety: usize,
    pub police: usize,
    pub religious: usize,
    pub accessibility: usize,
}
);

serde_clone_default!(
pub struct Scores {
    pub overall: f64,
    pub services: f64,
    pub mobility: f64,
    pub safety: f64,
    pub environment: f64,
}
);

serde_clone!(
pub struct Facility {
    pub id: String,
    pub name: String,
    pub category: String,
    pub lng: f64,
    pub lat: f64,
    pub distance: f64,
    pub contribution: f64,
    pub tags: Option<HashMap<String, String>>,
}
);

serde_only!(
pub struct CalculateScoreRequest {
    pub locations: Vec<SingleLocationRequest>,
}
);

serde_clone!(
pub struct SingleLocationRequest {
    pub lat: f64,
    pub lng: f64,
}
);

serde_only!(
pub struct OverpassResponse {
    pub elements: Vec<OverpassElement>,
}
);

serde_clone!(
pub struct OverpassElement {
    pub id: u64,
    #[serde(rename = "type")]
    pub element_type: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub center: Option<Center>,
    pub tags: Option<HashMap<String, String>>,
}
);

serde_clone!(
pub struct Center {
    pub lat: f64,
    pub lon: f64,
}
);
