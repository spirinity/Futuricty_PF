use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

pub static PATTERN_CONFIG: Lazy<Value> = Lazy::new(|| {
    std::fs::read_to_string("config/category_patterns.json")
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or(json!({}))
});

fn get_tag_as_str<'a>(tags: &'a HashMap<String, String>, key: &str) -> &'a str {
    tags.get(key).map(|s| s.as_str()).unwrap_or("")
}

fn get_list(config: &Value, category: &str, key: &str, default: &[&str]) -> Vec<String> {
    config
        .get(category)
        .and_then(|cat| cat.get(key))
        .and_then(|arr| arr.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_else(|| default.iter().map(|s| s.to_string()).collect())
}

fn get_bool(config: &Value, category: &str, key: &str, default: bool) -> bool {
    config
        .get(category)
        .and_then(|cat| cat.get(key))
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

fn name_contains_any(name: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|p| name.contains(p))
}

fn is_education(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let amenity_list = get_list(config, "education", "amenity_equals", &[]);
    let name_list = get_list(config, "education", "name_contains", &[]);

    amenity_list.iter().any(|a| a == amenity) || name_contains_any(name, &name_list)
}

fn is_police(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let amenity_list = get_list(config, "police", "amenity_equals", &[]);
    let name_list = get_list(config, "police", "name_contains", &[]);

    amenity_list.iter().any(|a| a == amenity) || name_contains_any(name, &name_list)
}

fn is_market(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let shop = get_tag_as_str(tags, "shop");

    let amenity_list = get_list(config, "market", "amenity_equals", &[]);
    let name_list = get_list(config, "market", "name_contains", &[]);
    let shop_flag = get_bool(config, "market", "shop_non_empty", true);

    (shop_flag && !shop.is_empty())
        || amenity_list.iter().any(|a| a == amenity)
        || name_contains_any(name, &name_list)
}

fn is_health(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let amenity_list = get_list(config, "health", "amenity_equals", &[]);
    let name_list = get_list(config, "health", "name_contains", &[]);
    let prefix_list = get_list(config, "health", "name_prefixes", &[]);
    let exclude_prefix = get_list(config, "health", "name_prefix_exclude", &[]);

    let prefix_match = prefix_list
        .iter()
        .any(|p| name.starts_with(p) && !exclude_prefix.iter().any(|e| name.contains(e)));

    amenity_list.iter().any(|a| a == amenity) || prefix_match || name_contains_any(name, &name_list)
}

fn is_transport(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let public_transport = get_tag_as_str(tags, "public_transport");
    let highway = get_tag_as_str(tags, "highway");
    let railway = get_tag_as_str(tags, "railway");

    let pt_list = get_list(config, "transport", "public_transport_equals", &[]);
    let highway_list = get_list(config, "transport", "highway_equals", &[]);
    let railway_list = get_list(config, "transport", "railway_equals", &[]);
    let name_list = get_list(config, "transport", "name_contains", &[]);

    pt_list.iter().any(|v| v == public_transport)
        || highway_list.iter().any(|v| v == highway)
        || railway_list.iter().any(|v| v == railway)
        || name_contains_any(name, &name_list)
}

fn is_religious(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let amenity_list = get_list(config, "religious", "amenity_equals", &[]);
    let name_list = get_list(config, "religious", "name_contains", &[]);

    amenity_list.iter().any(|a| a == amenity) || name_contains_any(name, &name_list)
}

fn is_recreation(config: &Value, tags: &HashMap<String, String>, name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let leisure = get_tag_as_str(tags, "leisure");

    let leisure_list = get_list(config, "recreation", "leisure_equals", &[]);
    let amenity_list = get_list(config, "recreation", "amenity_equals", &[]);
    let name_list = get_list(config, "recreation", "name_contains", &[]);

    leisure_list.iter().any(|v| v == leisure)
        || amenity_list.iter().any(|v| v == amenity)
        || name_contains_any(name, &name_list)
}

fn is_walkability(config: &Value, tags: &HashMap<String, String>, _name: &str) -> bool {
    let highway = get_tag_as_str(tags, "highway");
    let amenity = get_tag_as_str(tags, "amenity");
    let route = get_tag_as_str(tags, "route");
    let traffic_calming = get_tag_as_str(tags, "traffic_calming");
    let lit = get_tag_as_str(tags, "lit");
    let natural = get_tag_as_str(tags, "natural");
    let landuse = get_tag_as_str(tags, "landuse");

    let highway_list = get_list(config, "walkability", "highway_equals", &[]);
    let route_list = get_list(config, "walkability", "route_equals", &[]);
    let amenity_list = get_list(config, "walkability", "amenity_equals", &[]);
    let natural_list = get_list(config, "walkability", "natural_equals", &[]);
    let landuse_list = get_list(config, "walkability", "landuse_equals", &[]);
    let lit_yes = get_bool(config, "walkability", "lit_yes", true);
    let traffic_flag = get_bool(config, "walkability", "traffic_calming_present", true);

    highway_list.iter().any(|v| v == highway)
        || route_list.iter().any(|v| v == route)
        || amenity_list.iter().any(|v| v == amenity)
        || (lit_yes && lit == "yes")
        || (traffic_flag && !traffic_calming.is_empty())
        || natural_list.iter().any(|v| v == natural)
        || landuse_list.iter().any(|v| v == landuse)
}

fn is_accessibility(config: &Value, tags: &HashMap<String, String>, _name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let barrier = get_tag_as_str(tags, "barrier");
    let kerb = get_tag_as_str(tags, "kerb");
    let highway = get_tag_as_str(tags, "highway");
    let wheelchair = get_tag_as_str(tags, "wheelchair");

    let barrier_list = get_list(config, "accessibility", "barrier_equals", &[]);
    let kerb_list = get_list(config, "accessibility", "kerb_equals", &[]);
    let highway_list = get_list(config, "accessibility", "highway_equals", &[]);
    let wheelchair_list = get_list(config, "accessibility", "wheelchair_equals", &[]);
    let amenity_list = get_list(config, "accessibility", "amenity_equals", &[]);
    let tactile_yes = get_bool(config, "accessibility", "tactile_paving_yes", true);
    let tactile_paving = tags.get("tactile_paving")
        .map(|s| s.as_str())
        .map(|s| s == "yes")
        .unwrap_or(false);

    barrier_list.iter().any(|v| v == barrier)
        || kerb_list.iter().any(|v| v == kerb)
        || highway_list.iter().any(|v| v == highway)
        || wheelchair_list.iter().any(|v| v == wheelchair)
        || amenity_list.iter().any(|v| v == amenity)
        || (tactile_yes && tactile_paving)
}

fn is_safety(config: &Value, tags: &HashMap<String, String>, _name: &str) -> bool {
    let amenity = get_tag_as_str(tags, "amenity");
    let highway = get_tag_as_str(tags, "highway");
    let lit = get_tag_as_str(tags, "lit");
    let traffic_calming = get_tag_as_str(tags, "traffic_calming");
    let man_made = get_tag_as_str(tags, "man_made");

    let highway_list = get_list(config, "safety", "highway_equals", &[]);
    let lit_yes = get_bool(config, "safety", "lit_yes", true);
    let traffic_flag = get_bool(config, "safety", "traffic_calming_present", true);
    let man_made_list = get_list(config, "safety", "man_made_equals", &[]);
    let amenity_list = get_list(config, "safety", "amenity_equals", &[]);

    highway_list.iter().any(|v| v == highway)
        || (lit_yes && lit == "yes")
        || (traffic_flag && !traffic_calming.is_empty())
        || man_made_list.iter().any(|v| v == man_made)
        || amenity_list.iter().any(|v| v == amenity)
}

pub fn detect_category(tags: &HashMap<String, String>, raw_name: &str) -> Option<&'static str> {
    let name = raw_name.to_lowercase();
    let cfg = &PATTERN_CONFIG;

    type Detector = fn(&Value, &HashMap<String, String>, &str) -> bool;
    
    let detectors: [(Detector, &'static str); 10] = [
        (is_education, "education"),
        (is_police, "police"),
        (is_market, "market"),
        (is_health, "health"),
        (is_transport, "transport"),
        (is_religious, "religious"),
        (is_recreation, "recreation"),
        (is_walkability, "walkability"),
        (is_accessibility, "accessibility"),
        (is_safety, "safety"),
    ];

    detectors
        .iter()
        .find(|(detector, _)| detector(cfg, tags, &name))
        .map(|(_, category)| *category)
}
