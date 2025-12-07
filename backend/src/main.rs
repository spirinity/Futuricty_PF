mod models;
mod services;

use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use futures::stream::{self, StreamExt};
use std::collections::HashSet;
use std::sync::Arc;
use serde_json::{json, Value};
use once_cell::sync::Lazy;

use crate::models::{CalculateScoreRequest, LocationData};
use crate::services::overpass::OverpassService;

use crate::services::score_calculator::{process_facilities, calculate_scores};

pub static QUERY_CONFIG: Lazy<Value> = Lazy::new(|| {
    std::fs::read_to_string("config/queries.json")
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or(json!({}))
});

const MAX_FACILITY_DISTANCE: f64 = 500.0; 
const SEARCH_RADIUS: i32 = 500;
const MAX_NEARBY_FACILITIES: usize = 10;
const RATE_LIMIT_DELAY_SECS: u64 = 2;

struct DeduplicationState {
    facilities: Vec<crate::models::Facility>,
    seen_ids: HashSet<String>,
}

impl DeduplicationState {
    fn new() -> Self {
        DeduplicationState {
            facilities: Vec::new(),
            seen_ids: HashSet::new(),
        }
    }

    fn add_facility(mut self, facility: crate::models::Facility) -> Self {
        if self.seen_ids.insert(facility.id.clone()) {
            self.facilities.push(facility);
        }
        self
    }

    fn into_unique_facilities(self) -> Vec<crate::models::Facility> {
        self.facilities
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/calculate-score", post(calculate_score))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Server runtime error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}

async fn root() -> &'static str {
    "Futuricity Backend is running!"
}

pub async fn calculate_score(
    Json(payload): Json<CalculateScoreRequest>
) -> Result<Json<Vec<LocationData>>, (StatusCode, String)> {
    if payload.locations.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Locations array cannot be empty".to_string()));
    }

    payload.locations.iter()
        .find_map(|loc| -> Option<Result<(), (StatusCode, String)>> {
            if !(loc.lat >= -90.0 && loc.lat <= 90.0) {
                Some(Err((StatusCode::BAD_REQUEST, 
                    format!("Invalid latitude: {}. Must be between -90 and 90", loc.lat))))
            } else if !(loc.lng >= -180.0 && loc.lng <= 180.0) {
                Some(Err((StatusCode::BAD_REQUEST, 
                    format!("Invalid longitude: {}. Must be between -180 and 180", loc.lng))))
            } else {
                None
            }
        })
        .transpose()?;

    let overpass_service = Arc::new(OverpassService::new());

    let location_data_list = stream::iter(payload.locations)
        .then(|loc| {
            let service = overpass_service.clone();
            async move {
                let categories = vec![
                    "health", "education", "market", "transport", "walkability",
                    "recreation", "safety", "police", "religious", "accessibility"
                ];

                let queries: Vec<(String, String)> = categories.iter().map(|&cat| {
                    let query = generate_overpass_query(cat, loc.lat, loc.lng);
                    (cat.to_string(), query)
                }).collect();

                let facilities_data = service.fetch_facilities(queries).await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                let all_facilities = facilities_data.into_iter() 
                    .flat_map(|(_category, elements)| {
                        process_facilities(&elements, loc.lat, loc.lng)
                    })
                    .filter(|f| f.distance <= MAX_FACILITY_DISTANCE)
                    .fold(DeduplicationState::new(), |state, facility| state.add_facility(facility))
                    .into_unique_facilities();

                let (scores, facility_counts) = calculate_scores(&all_facilities);

                let nearby_facilities: Vec<String> = all_facilities.iter()
                    .take(MAX_NEARBY_FACILITIES)
                    .map(|f| f.name.as_str())
                    .map(|s| s.to_string())
                    .collect();
                
                tokio::time::sleep(std::time::Duration::from_secs(RATE_LIMIT_DELAY_SECS)).await;

                Ok(LocationData {   
                    address: format!("{}, {}", loc.lat, loc.lng),
                    facility_counts,
                    scores,
                    nearby_facilities,
                    facilities: all_facilities,
                })
            }
        })
        .collect::<Vec<Result<LocationData, (StatusCode, String)>>>()
        .await
        .into_iter()
        .collect::<Result<Vec<LocationData>, (StatusCode, String)>>()?;

    Ok(Json(location_data_list))
}

fn parse_config_key(key: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}

fn build_single_query(
    element_type: &str,
    attribute: &str,
    tags_str: &str,
    distance: i32,
    lat: f64,
    lng: f64,
) -> String {
    format!(
        r#"{}["{}"~"^({})$"](around:{},{},{});"#,
        element_type, attribute, tags_str, distance, lat, lng
    )
}

fn extract_queries_from_config(
    category_config: &Value,
    lat: f64,
    lng: f64,
    distance: i32,
) -> Vec<String> {
    category_config
        .as_object()
        .map(|obj| {
            obj.iter()
                .filter(|(key, _)| *key != "description")
                .filter_map(|(key, value)| {
                    let tags_str = value.as_str()?;
                    if tags_str.is_empty() {
                        return None;
                    }
                    
                    let (element_type, attribute) = parse_config_key(key)?;
                    Some(build_single_query(element_type, attribute, tags_str, distance, lat, lng))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn build_query_from_config(category: &str, lat: f64, lng: f64, distance: i32) -> Option<String> {
    QUERY_CONFIG
        .get("queries")
        .and_then(|queries| queries.get(category))
        .map(|category_config| extract_queries_from_config(category_config, lat, lng, distance))
        .and_then(|queries| {
            if queries.is_empty() {
                None
            } else {
                Some(queries.join(" "))
            }
        })
}

fn generate_overpass_query(category: &str, lat: f64, lng: f64) -> String {
    let distance = SEARCH_RADIUS;

    let query_body = build_query_from_config(category, lat, lng, distance)
        .unwrap_or_else(|| {
            eprintln!("WARNING: Category '{}' not found in config, using default query", category);
            format!(r#"node["amenity"](around:{},{},{});"#, distance, lat, lng)
        });

    format!(r#"[out:json];({});out center;"#, query_body)
}

