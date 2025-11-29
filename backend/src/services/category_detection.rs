use std::collections::HashMap;


pub fn detect_category(tags: &HashMap<String, String>, raw_name: &str) -> &'static str {
    let name = raw_name.to_lowercase();

    let amenity = tags.get("amenity").map(|s| s.as_str()).unwrap_or("");
    let shop = tags.get("shop").map(|s| s.as_str()).unwrap_or("");
    let leisure = tags.get("leisure").map(|s| s.as_str()).unwrap_or("");
    let highway = tags.get("highway").map(|s| s.as_str()).unwrap_or("");
    let railway = tags.get("railway").map(|s| s.as_str()).unwrap_or("");
    let public_transport = tags.get("public_transport").map(|s| s.as_str()).unwrap_or("");
    let barrier = tags.get("barrier").map(|s| s.as_str()).unwrap_or("");
    let kerb = tags.get("kerb").map(|s| s.as_str()).unwrap_or("");
    let natural = tags.get("natural").map(|s| s.as_str()).unwrap_or("");
    let landuse = tags.get("landuse").map(|s| s.as_str()).unwrap_or("");
    let route = tags.get("route").map(|s| s.as_str()).unwrap_or("");
    let lit = tags.get("lit").map(|s| s.as_str()).unwrap_or("");
    let traffic_calming = tags.get("traffic_calming").map(|s| s.as_str()).unwrap_or("");
    let crossing = tags.get("crossing").map(|s| s.as_str()).unwrap_or("");
    let zone_traffic = tags.get("zone:traffic").map(|s| s.as_str()).unwrap_or("");
    let wheelchair = tags.get("wheelchair").map(|s| s.as_str()).unwrap_or("");
    let man_made = tags.get("man_made").map(|s| s.as_str()).unwrap_or("");

    if matches!(amenity, "school" | "university" | "college" | "kindergarten" | "library")
        || name.contains("sekolah")
        || name.contains("sma")
        || name.contains("smp")
        || name.contains("sd ")
        || name.contains("smk")
        || name.contains("universitas")
        || name.contains("univ")
        || name.contains("kampus")
        || name.contains("tk")
        || name.contains("paud")
        || name.contains("perpustakaan")
        || name.contains("library")
    {
        return "education";
    }

    if amenity == "police"
        || name.contains("polisi")
        || name.contains("polres")
        || name.contains("polsek")
        || name.contains("polda")
        || name.contains("satlantas")
        || name.contains("satpol")
        || name.contains("police")
    {
        return "police";
    }

    if !shop.is_empty()
        || matches!(amenity, 
            "restaurant" | "cafe" | "fast_food" | "food_court" | "bar" | "pub" | "ice_cream" | "coffee_shop"
        )
        || matches!(amenity, "fuel" | "gas_station" | "petrol_station" | "service_station")
        || name.contains("spbu")
        || name.contains("pom bensin")
        || name.contains("gas station")
        || name.contains("pertamina")
        || name.contains("shell")
        || name.contains("warung")
        || name.contains("toko")
        || name.contains("shop")
        || name.contains("store")
        || name.contains("market")
        || name.contains("mall")
        || name.contains("plaza")
    {
        return "market";
    }

    if matches!(amenity, 
        "hospital" | "clinic" | "doctors" | "dentist" | "pharmacy" | "veterinary"
    )
        || name.contains("rumah sakit")
        || (name.starts_with("rs ") && !name.contains("sekolah"))
        || name.contains("rsud")
        || name.contains("klinik")
        || name.contains("apotek")
        || name.contains("dokter")
        || name.contains("puskesmas")
        || name.contains("poli")
    {
        return "health";
    }

    if matches!(public_transport, "platform" | "station" | "stop_position")
        || highway == "bus_stop"
        || matches!(railway, "station" | "halt" | "tram_stop")
        || name.contains("halte")
        || name.contains("bus stop")
        || name.contains("terminal")
        || name.contains("stasiun")
        || name.contains("station")
        || name.contains("mrt")
        || name.contains("lrt")
        || name.contains("angkot")
    {
        return "transport";
    }

    if matches!(
        amenity,
        "place_of_worship" | "mosque" | "church" | "temple" | "synagogue"
            | "hindu_temple" | "buddhist_temple"
    ) || name.contains("masjid")
        || name.contains("gereja")
        || name.contains("katedral")
        || name.contains("pura")
        || name.contains("vihara")
        || name.contains("candi")
    {
        return "religious";
    }

    if matches!(
        leisure,
        "park" | "playground" | "sports_centre" | "fitness_centre" | "swimming_pool" | "garden"
    ) || matches!(amenity, "cinema" | "theatre")
        || name.contains("taman")
        || name.contains("gym")
        || name.contains("fitness")
        || name.contains("playground")
        || name.contains("bioskop")
        || name.contains("cinema")
        || name.contains("kolam renang")
    {
        return "recreation";
    }

    if matches!(highway, "footway" | "pedestrian" | "path" | "steps")
        || matches!(route, "foot" | "hiking" | "walking")
        || amenity == "bench"
        || amenity == "drinking_water"
        || highway == "street_lamp"
        || !traffic_calming.is_empty()
        || lit == "yes"
        || natural == "tree_row"
        || natural == "hedge"
        || landuse == "grass"
        || landuse == "meadow"
        || (highway == "crossing")
    {
        return "walkability";
    }

    if barrier == "kerb"
        || kerb == "lowered"
        || kerb == "flush"
        || highway == "elevator"
        || wheelchair == "yes"
        || amenity == "toilets"
        || tags.get("tactile_paving").map(|s| s == "yes").unwrap_or(false)
    {
        return "accessibility";
    }

    if highway == "street_lamp"
        || lit == "yes"
        || !traffic_calming.is_empty()
        || man_made == "surveillance"
        || matches!(amenity, "fire_station" | "hospital")
    {
        return "safety";
    }

    "market"
}
