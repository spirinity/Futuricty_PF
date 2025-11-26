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

use crate::models::{CalculateScoreRequest, LocationData};
use crate::services::overpass::OverpassService;

// FUNCTION IMPORT
use crate::services::score_calculator::{process_facilities, calculate_scores};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/calculate-score", post(calculate_score))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Futuricity Backend is running!"
}

async fn calculate_score(
    Json(payload): Json<CalculateScoreRequest>,
) -> Result<Json<LocationData>, (StatusCode, String)> {

    let overpass_service = OverpassService::new();

    let categories = vec![
        "health", "education", "market", "transport", "walkability",
        "recreation", "safety", "police", "religious", "accessibility"
    ];

    let queries: Vec<(String, String)> = categories.iter().map(|&cat| {
        let query = generate_overpass_query(cat, payload.lat, payload.lng);
        (cat.to_string(), query)
    }).collect();

    let facilities_data = overpass_service.fetch_facilities(queries).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut all_facilities = Vec::new();

    for (category, elements) in facilities_data {
        let processed = process_facilities(&elements, &category, payload.lat, payload.lng);
        all_facilities.extend(processed);
    }

    let (scores, facility_counts) = calculate_scores(&all_facilities);

    let nearby_facilities: Vec<String> = all_facilities.iter()
        .take(10)
        .map(|f| f.name.clone())
        .collect();

    let location_data = LocationData {
        address: format!("{}, {}", payload.lat, payload.lng),
        facility_counts,
        scores,
        nearby_facilities,
        facilities: all_facilities,
    };

    Ok(Json(location_data))
}

fn generate_overpass_query(category: &str, lat: f64, lng: f64) -> String {
    let distance = 1000;

    let query_body = match category {
        "health" => format!(r#"
            node["amenity"~"^(hospital|clinic|doctors|dentist|pharmacy)$"](around:{},{lat},{lng});
            way["amenity"~"^(hospital|clinic|doctors|dentist|pharmacy)$"](around:{},{lat},{lng});
        "#, distance, distance),
        "education" => format!(r#"
            node["amenity"~"^(school|university|college|kindergarten)$"](around:{},{lat},{lng});
            way["amenity"~"^(school|university|college|kindergarten)$"](around:{},{lat},{lng});
        "#, distance, distance),
        "market" => format!(r#"
            node["shop"](around:{},{lat},{lng});
            node["amenity"~"^(restaurant|cafe)$"](around:{},{lat},{lng});
        "#, distance, distance),
        _ => format!(r#"
            node["amenity"](around:{},{lat},{lng});
        "#, distance),
    };

    format!(r#"[out:json];({});out center;"#, query_body)
        .replace("{lat}", &lat.to_string())
        .replace("{lng}", &lng.to_string())
}
