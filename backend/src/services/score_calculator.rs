use crate::models::{Facility, FacilityCounts, Scores, OverpassElement};
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::services::category_detection::detect_category;

/// Earth radius in meters
const R: f64 = 6_371_000.0;

/// Haversine distance calculation
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

/// Contribution formula
fn calculate_contribution(distance: f64, category: &str) -> f64 {
    // Max radius 1000m
    if distance > 1000.0 {
        return 0.0;
    }

    let (max_contribution, decay) = match category {
        "health" => (10.0, 0.8),
        "education" => (10.0, 0.9),
        "market" => (8.0, 0.85),
        "transport" => (10.0, 0.95),
        "walkability" => (12.0, 0.85),
        "recreation" => (8.0, 0.8),
        "safety" => (6.0, 0.7),
        "accessibility" => (4.0, 0.9),
        "police" => (8.0, 0.6),
        "religious" => (6.0, 0.75),
        _ => (5.0, 0.8),
    };

    let norm = distance / 1000.0;
    let contribution = max_contribution * (1.0 - norm).powf(decay);

    // Minimum contribution biar gak nol
    let min_contribution = max_contribution * 0.1;
    contribution.max(min_contribution)
}

/// Process facilities using Rayon — final version
pub fn process_facilities(
    elements: &[OverpassElement],
    category: &str,
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

            let name = tags_ref
                .get("name")
                .or_else(|| tags_ref.get("amenity"))
                .or_else(|| tags_ref.get("shop"))
                .or_else(|| tags_ref.get("leisure"))
                .or_else(|| tags_ref.get("highway"))
                .cloned()
                .unwrap_or_else(|| format!("{category} facility"));

            let actual_category = detect_category(tags_ref, &name);

            let contribution = calculate_contribution(distance, actual_category);

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

/// FINAL Calculate total scores + counts
pub fn calculate_scores(facilities: &[Facility]) -> (Scores, FacilityCounts) {
    let mut counts = FacilityCounts::default();
    let mut map: HashMap<String, f64> = HashMap::new();

    for f in facilities {
        match f.category.as_str() {
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

        *map.entry(f.category.clone()).or_insert(0.0) += f.contribution;
    }

    // Subscores
    let services =
        map.get("health").unwrap_or(&0.0)
        + map.get("education").unwrap_or(&0.0)
        + map.get("market").unwrap_or(&0.0)
        + map.get("religious").unwrap_or(&0.0);

    let mobility =
        map.get("transport").unwrap_or(&0.0)
        + map.get("walkability").unwrap_or(&0.0);

    let safety =
        map.get("safety").unwrap_or(&0.0)
        + map.get("police").unwrap_or(&0.0)
        + map.get("accessibility").unwrap_or(&0.0);

    let environment =
        *map.get("recreation").unwrap_or(&0.0);

    let normalize = |v: f64| v.clamp(0.0, 100.0);

    let scores = Scores {
        services: normalize(services),
        mobility: normalize(mobility),
        safety: normalize(safety),
        environment: normalize(environment),
        overall: normalize((services + mobility + safety + environment) / 4.0),
    };

    (scores, counts)
}
