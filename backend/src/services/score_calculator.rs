use crate::models::{Facility, FacilityCounts, Scores, OverpassElement};
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde_json::{json, Value};

use crate::services::category_detection::detect_category;

/// Scoring configuration loaded from JSON file
pub static SCORING_CONFIG: Lazy<Value> = Lazy::new(|| {
    std::fs::read_to_string("config/scoring_config.json")
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or(json!({}))
});

const R: f64 = 6_371_000.0;  // Earth radius in meters
const MAX_FACILITY_DISTANCE: f64 = 500.0;  // Maximum distance for facility contribution (meters)

/// Get contribution weights for a category from config
fn get_contribution_weights(config: &Value, category: &str) -> (f64, f64, f64) {
    let weights = &config["contribution_weights"];
    let cat_weights = weights.get(category).or_else(|| weights.get("default"));
    
    if let Some(w) = cat_weights {
        let max_contrib = w["max_contribution"].as_f64().unwrap_or(10.0);
        let decay = w["decay_factor"].as_f64().unwrap_or(0.8);
        let min_ratio = w["min_contribution_ratio"].as_f64().unwrap_or(0.1);
        (max_contrib, decay, min_ratio)
    } else {
        (10.0, 0.8, 0.1)  // Default fallback
    }
}

/// Get score weights from config
fn get_score_weights(config: &Value) -> (f64, f64, f64, f64, f64) {
    let weights = &config["score_weights"];
    // Fallback values matching config/scoring_config.json
    let services_w = weights["services_weight"].as_f64().unwrap_or(0.3);
    let mobility_w = weights["mobility_weight"].as_f64().unwrap_or(0.25);
    let safety_w = weights["safety_weight"].as_f64().unwrap_or(0.25);
    let environment_w = weights["environment_weight"].as_f64().unwrap_or(0.2);
    let health_to_safety = weights["health_contribution_to_safety"].as_f64().unwrap_or(0.5);
    
    (services_w, mobility_w, safety_w, environment_w, health_to_safety)
}

/// Get score clamping values from config
fn get_score_clamps(config: &Value) -> (f64, f64) {
    let weights = &config["score_weights"];
    let min = weights["score_clamp_min"].as_f64().unwrap_or(0.0);
    let max = weights["score_clamp_max"].as_f64().unwrap_or(100.0);
    (min, max)
}

/// Get category mappings from config
fn get_category_mappings(config: &Value) -> HashMap<String, Vec<String>> {
    let mappings = &config["category_mappings"];
    let mut result = HashMap::new();
    
    for (key, categories) in mappings.as_object().unwrap_or(&Default::default()) {
        let cats: Vec<String> = categories
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
        result.insert(key.clone(), cats);
    }
    result
}

/// Extract facility name from tags using config-driven field priority
fn extract_facility_name(tags: &HashMap<String, String>, config: &Value) -> String {
    let name_config = &config["name_extraction"];
    
    // Get fallback fields from config
    let fallback_fields: Vec<String> = name_config["fallback_fields"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_else(|| vec!["name".to_string(), "amenity".to_string()]);
    
    // Try each field in order
    for field in &fallback_fields {
        if let Some(value) = tags.get(field) {
            return value.clone();
        }
    }
    
    // Use default name from config
    name_config["default_name"]
        .as_str()
        .unwrap_or("facility")
        .to_string()
}

/// Calculate score for a category group using config mappings
fn calculate_category_group_score(
    map: &HashMap<String, f64>,
    categories: &[String],
    health_contribution: f64,
    health_to_safety: f64,
) -> f64 {
    // Special handling for safety group (includes health contribution)
    if categories.contains(&"health".to_string()) && health_contribution > 0.0 {
        let base: f64 = categories
            .iter()
            .map(|cat| *map.get(cat).unwrap_or(&0.0))
            .sum();
        return base + (health_contribution * health_to_safety);
    }
    
    categories
        .iter()
        .map(|cat| *map.get(cat).unwrap_or(&0.0))
        .sum()
}

fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos()
            * lat2.to_radians().cos()
            * (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    R * c
}

/// Pure function to check if distance is within threshold
fn is_within_distance_threshold(distance: f64) -> bool {
    distance <= MAX_FACILITY_DISTANCE
}

/// Pure function to normalize distance (0.0 to 1.0)
fn normalize_distance(distance: f64) -> f64 {
    distance / MAX_FACILITY_DISTANCE
}
/// Calculate contribution score with configurable weights
fn calculate_contribution(distance: f64, category: &str, config: &Value) -> f64 {
    if !is_within_distance_threshold(distance) {
        return 0.0;
    }

    let (max_contribution, decay, min_ratio) = get_contribution_weights(config, category);

    let norm = normalize_distance(distance);
    let contribution = max_contribution * (1.0 - norm).powf(decay);

    let min_contribution = max_contribution * min_ratio;
    contribution.max(min_contribution)
}

pub fn process_facilities(
    elements: &[OverpassElement],
    user_lat: f64,
    user_lng: f64,
) -> Vec<Facility> {
    static EMPTY: Lazy<HashMap<String, String>> = Lazy::new(|| HashMap::new());

    elements
        .par_iter()
        .filter_map(|element| {
            let lat = element.lat.or_else(|| element.center.as_ref().map(|c| c.lat))?;
            let lng = element.lon.or_else(|| element.center.as_ref().map(|c| c.lon))?;

            let distance = calculate_distance(user_lat, user_lng, lat, lng);

            let tags_ref = element.tags.as_ref().unwrap_or(&EMPTY);

            let name = extract_facility_name(tags_ref, &SCORING_CONFIG);

            let actual_category = detect_category(tags_ref, &name)?;

            let contribution = calculate_contribution(distance, actual_category, &SCORING_CONFIG);

            if contribution <= 0.0 {
                return None;
            }

            Some(Facility {
                id: format!("{}-{}", actual_category, element.id),
                name,
                category: actual_category.to_string(),
                lng,
                lat,
                distance,
                contribution,
                tags: element.tags.clone(),
            })
        })
        .collect()
}

/// Pure function untuk increment category count secara immutable
fn increment_category_count(mut counts: FacilityCounts, category: &str) -> FacilityCounts {
    match category {
        "health" => counts.health += 1,
        "education" => counts.education += 1,
        "market" => counts.market += 1,
        "transport" => counts.transport += 1,
        "walkability" => counts.walkability += 1,
        "recreation" => counts.recreation += 1,
        "safety" => counts.safety += 1,
        "police" => counts.police += 1,
        "religious" => counts.religious += 1,
        "accessibility" => counts.accessibility += 1,
        _ => {}
    }
    counts
}

/// Pure function untuk update contribution map secara immutable
fn update_contribution_map(
    mut map: HashMap<String, f64>,
    category: &str,
    contribution: f64,
) -> HashMap<String, f64> {
    *map.entry(category.to_string()).or_insert(0.0) += contribution;
    map
}

pub fn calculate_scores(facilities: &[Facility]) -> (Scores, FacilityCounts) {
    // Pure functional approach: no mut in fold closure
    let (counts, map) = facilities.iter().fold(
        (FacilityCounts::default(), HashMap::new()),
        |(counts, map), f| {
            let new_counts = increment_category_count(counts, &f.category);
            let new_map = update_contribution_map(map, &f.category, f.contribution);
            (new_counts, new_map)
        },
    );

    // Load config-driven values
    let (clamp_min, clamp_max) = get_score_clamps(&SCORING_CONFIG);
    let normalize = |v: f64| v.clamp(clamp_min, clamp_max);
    
    let category_mappings = get_category_mappings(&SCORING_CONFIG);
    let (services_w, mobility_w, safety_w, environment_w, health_to_safety) = get_score_weights(&SCORING_CONFIG);
    let health_contribution = *map.get("health").unwrap_or(&0.0);

    // Calculate scores using config mappings
    let services_score = category_mappings
        .get("services")
        .map(|cats| calculate_category_group_score(&map, cats, health_contribution, health_to_safety))
        .unwrap_or(0.0);

    let mobility_score = category_mappings
        .get("mobility")
        .map(|cats| calculate_category_group_score(&map, cats, health_contribution, health_to_safety))
        .unwrap_or(0.0);

    let safety_score = category_mappings
        .get("safety")
        .map(|cats| calculate_category_group_score(&map, cats, health_contribution, health_to_safety))
        .unwrap_or(0.0);

    let environment_score = category_mappings
        .get("environment")
        .map(|cats| calculate_category_group_score(&map, cats, health_contribution, health_to_safety))
        .unwrap_or(0.0);

    // Normalize individual scores
    let services_normalized = normalize(services_score);
    let mobility_normalized = normalize(mobility_score);
    let safety_normalized = normalize(safety_score);
    let environment_normalized = normalize(environment_score);

    // Calculate weighted overall score
    let overall = 
        (services_normalized * services_w) + 
        (mobility_normalized * mobility_w) + 
        (safety_normalized * safety_w) + 
        (environment_normalized * environment_w);

    let scores = Scores {
        services: services_normalized,
        mobility: mobility_normalized,
        safety: safety_normalized,
        environment: environment_normalized,
        overall: normalize(overall),
    };

    (scores, counts)
}
