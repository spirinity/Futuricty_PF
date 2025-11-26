use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationData {
    pub address: String,
    pub facility_counts: FacilityCounts,
    pub scores: Scores,
    pub nearby_facilities: Vec<String>,
    pub facilities: Vec<Facility>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Scores {
    pub overall: f64,
    pub services: f64,
    pub mobility: f64,
    pub safety: f64,
    pub environment: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CalculateScoreRequest {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverpassResponse {
    pub elements: Vec<OverpassElement>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OverpassElement {
    pub id: u64,
    #[serde(rename = "type")]
    pub element_type: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub center: Option<Center>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Center {
    pub lat: f64,
    pub lon: f64,
}
