use crate::models::{Facility, FacilityCounts, Scores, OverpassElement};
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde_json::{json, Value};

use crate::services::category_detection::detect_category;

pub static SCORING_CONFIG: Lazy<Value> = Lazy::new(|| {
    std::fs::read_to_string("config/scoring_config.json")
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or(json!({}))
});

const R: f64 = 6_371_000.0;
const MAX_FACILITY_DISTANCE: f64 = 500.0;

fn get_contribution_weights(config: &Value, category: &str) -> (f64, f64, f64) {
    const DEFAULT_MAX_CONTRIB: f64 = 10.0;
    const DEFAULT_DECAY: f64 = 0.8;
    const DEFAULT_MIN_RATIO: f64 = 0.1;
    
    config["contribution_weights"]
        .get(category)
        .or_else(|| config["contribution_weights"].get("default"))
        .and_then(|w| {
            let max_contrib = w["max_contribution"].as_f64()?;
            let decay = w["decay_factor"].as_f64()?;
            let min_ratio = w["min_contribution_ratio"].as_f64()?;
            Some((max_contrib, decay, min_ratio))
        })
        .unwrap_or((DEFAULT_MAX_CONTRIB, DEFAULT_DECAY, DEFAULT_MIN_RATIO))
}

fn get_score_weights(config: &Value) -> (f64, f64, f64, f64, f64) {
    const DEFAULT_SERVICES: f64 = 0.3;
    const DEFAULT_MOBILITY: f64 = 0.25;
    const DEFAULT_SAFETY: f64 = 0.25;
    const DEFAULT_ENVIRONMENT: f64 = 0.2;
    const DEFAULT_HEALTH_TO_SAFETY: f64 = 0.5;
    
    let weights = &config["score_weights"];
    
    (
        weights["services_weight"].as_f64().unwrap_or(DEFAULT_SERVICES),
        weights["mobility_weight"].as_f64().unwrap_or(DEFAULT_MOBILITY),
        weights["safety_weight"].as_f64().unwrap_or(DEFAULT_SAFETY),
        weights["environment_weight"].as_f64().unwrap_or(DEFAULT_ENVIRONMENT),
        weights["health_contribution_to_safety"].as_f64().unwrap_or(DEFAULT_HEALTH_TO_SAFETY),
    )
}

fn get_score_clamps(config: &Value) -> (f64, f64) {
    const DEFAULT_MIN: f64 = 0.0;
    const DEFAULT_MAX: f64 = 100.0;
    
    let weights = &config["score_weights"];
    (
        weights["score_clamp_min"].as_f64().unwrap_or(DEFAULT_MIN),
        weights["score_clamp_max"].as_f64().unwrap_or(DEFAULT_MAX),
    )
}

fn get_category_mappings(config: &Value) -> HashMap<String, Vec<String>> {
    config["category_mappings"]
        .as_object()
        .map(|mappings| {
            mappings
                .iter()
                .filter_map(|(key, categories)| {
                    categories
                        .as_array()
                        .map(|arr| {
                            let cats: Vec<String> = arr
                                .iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                            (key.clone(), cats)
                        })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn extract_facility_name(tags: &HashMap<String, String>, config: &Value) -> String {
    const DEFAULT_FALLBACKS: &[&str] = &["name", "amenity"];
    const DEFAULT_NAME: &str = "facility";
    
    let name_config = &config["name_extraction"];
    
    let fallback_fields: Vec<&str> = name_config["fallback_fields"]
        .as_array()
        .and_then(|arr| {
            let fields: Vec<&str> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .collect();
            if fields.is_empty() { None } else { Some(fields) }
        })
        .unwrap_or_else(|| DEFAULT_FALLBACKS.to_vec());
    
    fallback_fields
        .iter()
        .find_map(|field| tags.get(*field).cloned())
        .or_else(|| {
            name_config["default_name"]
                .as_str()
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| DEFAULT_NAME.to_string())
}

fn calculate_category_group_score(
    map: &HashMap<String, f64>,
    categories: &[String],
    health_contribution: f64,
    health_to_safety: f64,
) -> f64 {
      let base: f64 = categories
        .iter()
        .map(|cat| *map.get(cat).unwrap_or(&0.0))
        .sum();
    
    base + if categories.contains(&"health".to_string()) && health_contribution > 0.0 {
        health_contribution * health_to_safety
    } else {
        0.0
    }
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

fn is_within_distance_threshold(distance: f64) -> bool {
    distance <= MAX_FACILITY_DISTANCE
}

fn normalize_distance(distance: f64) -> f64 {
    distance / MAX_FACILITY_DISTANCE
}

fn calculate_contribution(distance: f64, category: &str, config: &Value) -> f64 {
    if is_within_distance_threshold(distance) {
        let (max_contribution, decay, min_ratio) = get_contribution_weights(config, category);
        let norm = normalize_distance(distance);
        let contribution = max_contribution * (1.0 - norm).powf(decay);
        let min_contribution = max_contribution * min_ratio;
        contribution.max(min_contribution)
    } else {
        0.0
    }
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

fn update_contribution_map(
    mut map: HashMap<String, f64>,
    category: &str,
    contribution: f64,
) -> HashMap<String, f64> {
    *map.entry(category.to_string()).or_insert(0.0) += contribution;
    map
}

pub fn calculate_scores(facilities: &[Facility]) -> (Scores, FacilityCounts) {
    let (counts, map) = facilities.iter().fold(
        (FacilityCounts::default(), HashMap::new()),
        |(counts, map), f| {
            let new_counts = increment_category_count(counts, &f.category);
            let new_map = update_contribution_map(map, &f.category, f.contribution);
            (new_counts, new_map)
        },
    );

    let (clamp_min, clamp_max) = get_score_clamps(&SCORING_CONFIG);
    let normalize = |v: f64| v.clamp(clamp_min, clamp_max);
    
    let category_mappings = get_category_mappings(&SCORING_CONFIG);
    let (services_w, mobility_w, safety_w, environment_w, health_to_safety) = get_score_weights(&SCORING_CONFIG);
    let health_contribution = *map.get("health").unwrap_or(&0.0);

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

    let services_normalized = normalize(services_score);
    let mobility_normalized = normalize(mobility_score);
    let safety_normalized = normalize(safety_score);
    let environment_normalized = normalize(environment_score);

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
