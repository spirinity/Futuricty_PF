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
) -> Result<Json<Vec<LocationData>>, (StatusCode, String)> {

    let overpass_service = std::sync::Arc::new(OverpassService::new());
    let mut location_data_list = Vec::new();

    for loc in payload.locations {
        let categories = vec![
            "health", "education", "market", "transport", "walkability",
            "recreation", "safety", "police", "religious", "accessibility"
        ];

        let queries: Vec<(String, String)> = categories.iter().map(|&cat| {
            let query = generate_overpass_query(cat, loc.lat, loc.lng);
            (cat.to_string(), query)
        }).collect();

        let facilities_data = overpass_service.fetch_facilities(queries).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let mut all_facilities = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for (category, elements) in facilities_data {
            let processed = process_facilities(&elements, &category, loc.lat, loc.lng);
            for facility in processed {
                if facility.distance <= 500.0 && seen_ids.insert(facility.id.clone()) {
                    all_facilities.push(facility);
                }
            }
        }

        let (scores, facility_counts) = calculate_scores(&all_facilities);

        let nearby_facilities: Vec<String> = all_facilities.iter()
            .take(10)
            .map(|f| f.name.clone())
            .collect();

        location_data_list.push(LocationData {
            address: format!("{}, {}", loc.lat, loc.lng),
            facility_counts,
            scores,
            nearby_facilities,
            facilities: all_facilities,
        });
        
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    Ok(Json(location_data_list))
}

fn generate_overpass_query(category: &str, lat: f64, lng: f64) -> String {
    let distance = 500;

    let query_body = match category {
        "health" => format!(r#"
            node["amenity"~"^(hospital|clinic|doctors|dentist|pharmacy|veterinary)$"](around:{},{lat},{lng});
            way["amenity"~"^(hospital|clinic|doctors|dentist|pharmacy|veterinary)$"](around:{},{lat},{lng});
            node["name"~"^(rumah sakit|rsud|klinik|apotek|apotik|dokter|puskesmas|poli)"](around:{},{lat},{lng});
            way["name"~"^(rumah sakit|rsud|klinik|apotek|apotik|dokter|puskesmas|poli)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance),
        "education" => format!(r#"
            node["amenity"~"^(school|university|college|kindergarten|library)$"](around:{},{lat},{lng});
            way["amenity"~"^(school|university|college|kindergarten|library)$"](around:{},{lat},{lng});
            node["name"~"^(sekolah|sd|smp|sma|smk|universitas|univ|kampus|tk|paud|perpustakaan|library)"](around:{},{lat},{lng});
            way["name"~"^(sekolah|sd|smp|sma|smk|universitas|univ|kampus|tk|paud|perpustakaan|library)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance),
        "market" => format!(r#"
            node["shop"](around:{},{lat},{lng});
            way["shop"](around:{},{lat},{lng});
            node["amenity"~"^(restaurant|cafe|fast_food|food_court|bar|pub|ice_cream|coffee_shop)$"](around:{},{lat},{lng});
            way["amenity"~"^(restaurant|cafe|fast_food|food_court|bar|pub|ice_cream|coffee_shop)$"](around:{},{lat},{lng});
            node["amenity"~"^(shop|store|market|retail|food|beverage)$"](around:{},{lat},{lng});
            way["amenity"~"^(shop|store|market|retail|food|beverage)$"](around:{},{lat},{lng});
            node["amenity"~"^(fuel|gas_station|petrol_station|service_station)$"](around:{},{lat},{lng});
            way["amenity"~"^(fuel|gas_station|petrol_station|service_station)$"](around:{},{lat},{lng});
            node["name"~"^(spbu|pom bensin|gas station|petrol|fuel|bensin|solar|pertamina|shell|bp|esso|caltex|toko|warung|shop|store|market|mall|plaza)"](around:{},{lat},{lng});
            way["name"~"^(spbu|pom bensin|gas station|petrol|fuel|bensin|solar|pertamina|shell|bp|esso|caltex|toko|warung|shop|store|market|mall|plaza)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance),
        "transport" => format!(r#"
            node["public_transport"~"^(platform|station|stop_position)$"](around:{},{lat},{lng});
            node["highway"="bus_stop"](around:{},{lat},{lng});
            node["railway"~"^(station|halt|tram_stop)$"](around:{},{lat},{lng});
            way["public_transport"~"^(platform|station)$"](around:{},{lat},{lng});
            node["name"~"^(halte|bus stop|terminal|stasiun|station|mrt|lrt|transjakarta|angkot)"](around:{},{lat},{lng});
            way["name"~"^(halte|bus stop|terminal|stasiun|station|mrt|lrt|transjakarta|angkot)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance, distance, distance),
        "walkability" => format!(r#"
            way["highway"~"^(footway|pedestrian|path|steps|bridleway)$"](around:{},{lat},{lng});
            way["highway"="residential"]["foot"="designated"](around:{},{lat},{lng});
            way["highway"="service"]["foot"="designated"](around:{},{lat},{lng});
            way["sidewalk"~"^(both|left|right|separate)$"](around:{},{lat},{lng});
            way["footway"="sidewalk"](around:{},{lat},{lng});
            way["footway"="crossing"](around:{},{lat},{lng});
            node["highway"="crossing"](around:{},{lat},{lng});
            way["pedestrian"="yes"](around:{},{lat},{lng});
            way["pedestrian"="designated"](around:{},{lat},{lng});
            way["pedestrian"="zone"](around:{},{lat},{lng});
            way["route"="foot"](around:{},{lat},{lng});
            way["route"="hiking"](around:{},{lat},{lng});
            way["route"="walking"](around:{},{lat},{lng});
            node["amenity"="bench"](around:{},{lat},{lng});
            way["amenity"="bench"](around:{},{lat},{lng});
            node["amenity"="drinking_water"](around:{},{lat},{lng});
            way["amenity"="drinking_water"](around:{},{lat},{lng});
            node["highway"="street_lamp"](around:{},{lat},{lng});
            way["highway"]["lit"="yes"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance),
        "recreation" => format!(r#"
            node["leisure"~"^(park|playground|sports_centre|fitness_centre|swimming_pool|garden)$"](around:{},{lat},{lng});
            way["leisure"~"^(park|playground|sports_centre|fitness_centre|swimming_pool|garden)$"](around:{},{lat},{lng});
            node["amenity"~"^(cinema|theatre)$"](around:{},{lat},{lng});
            way["amenity"~"^(cinema|theatre)$"](around:{},{lat},{lng});
            node["name"~"^(taman|park|playground|kolam renang|swimming|gym|fitness|bioskop|cinema|teater|theatre)"](around:{},{lat},{lng});
            way["name"~"^(taman|park|playground|kolam renang|swimming|gym|fitness|bioskop|cinema|teater|theatre)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance, distance, distance),
        "safety" => format!(r#"
            node["highway"="street_lamp"](around:{},{lat},{lng});
            way["highway"]["lit"="yes"](around:{},{lat},{lng});
            node["highway"="crossing"](around:{},{lat},{lng});
            node["highway"="traffic_signals"](around:{},{lat},{lng});
            way["traffic_calming"](around:{},{lat},{lng});
            way["sidewalk"](around:{},{lat},{lng});
            node["amenity"="fire_station"](around:{},{lat},{lng});
            way["amenity"="fire_station"](around:{},{lat},{lng});
            node["amenity"="hospital"](around:{},{lat},{lng});
            way["amenity"="hospital"](around:{},{lat},{lng});
            node["man_made"="surveillance"](around:{},{lat},{lng});
            way["man_made"="surveillance"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance),
        "police" => format!(r#"
            node["amenity"="police"](around:{},{lat},{lng});
            way["amenity"="police"](around:{},{lat},{lng});
            node["name"~"^(polisi|polres|polsek|polda|satlantas|satpol|pp|police)"](around:{},{lat},{lng});
            way["name"~"^(polisi|polres|polsek|polda|satlantas|satpol|pp|police)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance),
        "religious" => format!(r#"
            node["amenity"~"^(place_of_worship|mosque|church|temple|synagogue|hindu_temple|buddhist_temple)$"](around:{},{lat},{lng});
            way["amenity"~"^(place_of_worship|mosque|church|temple|synagogue|hindu_temple|buddhist_temple)$"](around:{},{lat},{lng});
            node["name"~"^(masjid|gudang|gereja|katedral|katedral|synagogue|hindu_temple|buddhist_temple)"](around:{},{lat},{lng});
            way["name"~"^(masjid|gudang|gereja|katedral|katedral|synagogue|hindu_temple|buddhist_temple)"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance),
        "accessibility" => format!(r#"
            node["barrier"="kerb"](around:{},{lat},{lng});
            way["barrier"="kerb"](around:{},{lat},{lng});
            node["kerb"="lowered"](around:{},{lat},{lng});
            way["kerb"="lowered"](around:{},{lat},{lng});
            node["highway"="elevator"](around:{},{lat},{lng});
            way["highway"="elevator"](around:{},{lat},{lng});
            node["amenity"="parking"]["access"="designated"](around:{},{lat},{lng});
            way["amenity"="parking"]["access"="designated"](around:{},{lat},{lng});
            node["tactile_paving"="yes"](around:{},{lat},{lng});
            way["tactile_paving"="yes"](around:{},{lat},{lng});
            node["amenity"="toilets"]["wheelchair"="yes"](around:{},{lat},{lng});
            way["amenity"="toilets"]["wheelchair"="yes"](around:{},{lat},{lng});
        "#, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance, distance),
        _ => format!(r#"
            node["amenity"](around:{},{lat},{lng});
        "#, distance),
    };

    format!(r#"[out:json];({});out center;"#, query_body)
        .replace("{lat}", &lat.to_string())
        .replace("{lng}", &lng.to_string())
}
