# **FUTURCITY**

## **_Pendekatan Functional Programming dengan Rust_**

**Aisyah Wilda Fauziah Amanda, Galuh Juliviana Romanita, Mahardika Arka, Olivia Dafina**

### Abstract

Futurcity merupakan aplikasi analisis livability yang dikembangkan untuk membantu pengguna memahami kualitas lingkungan hunian berdasarkan data spasial. Aplikasi ini mendukung perbandingan hingga tiga lokasi sekaligus dengan mempertimbangkan indikator seperti layanan publik, mobilitas, keamanan, dan lingkungan dalam radius 500 meter. Rust digunakan sebagai bahasa backend karena menawarkan performa tinggi dan keamanan memori, sementara Axum berperan sebagai framework API dengan dukungan Tokio untuk pemrosesan asynchronous dan Rayon untuk komputasi paralel. Frontend dibangun menggunakan React, MapLibre GL, React Query, dan TailwindCSS untuk menghadirkan visualisasi peta interaktif serta antarmuka yang responsif.

Prinsip functional programming diterapkan terutama pada proses transformasi data, modularisasi logika perhitungan, serta pipeline scoring yang deterministik dan mudah diuji. Pendekatan ini meningkatkan konsistensi, keandalan, dan pemeliharaan sistem. Dengan memadukan teknologi modern dan metode pengolahan data spasial berbasis fungsi, Futurcity mampu memberikan hasil analisis yang cepat, akurat, dan informatif bagi evaluasi kualitas lingkungan hunian.

### Introduction

Dengan meningkatnya kebutuhan masyarakat untuk memahami kualitas suatu lingkungan tempat tinggal, muncul kebutuhan akan aplikasi yang mampu menyajikan informasi _livability_ secara komprehensif dalam radius yang relevan. Proyek Futurcity dikembangkan untuk memberikan gambaran menyeluruh mengenai aspek-aspek penting sebuah wilayah melalui pemetaan fasilitas publik dan penghitungan skor indikator dalam radius 500 meter dari suatu lokasi. Tidak hanya satu titik, Futurcity dirancang untuk menganalisis tiga titk lokasi sekaligus, sehingga pengguna dapat membandingkan kualitas beberapa area secara paralel. Penggunaan Rust memberikan jaminan performa dan keamanan yang diperlukan untuk mengolah data spasial secara intensif. Selain itu, penerapan prinsip functional programming memudahkan pengembangan modul analisis yang bersifat modular dan mudah diuji. Keunikan Futurcity adalah kemampuannya mengintegrasikan data spasial real-time, indikator lingkungan hunian, dan visualisasi peta interaktif ke dalam satu platform analisis hunian yang komprehensif. Dengan pendekatan terstruktur berbasis indikator, Futurcity tidak hanya menampilkan lokasi, tetapi juga menyajikan penilaian kualitas lingkungan yang mudah dipahami oleh pengguna. Keunggulan ini diperkuat oleh arsitektur sistem yang memisahkan beban komputasi berat ke backend berkinerja tinggi, sehingga analisis mendalam tetap dapat disajikan melalui antarmuka yang responsif dan mulus.

### Background and Concepts

Futurcity dirancang sebagai aplikasi yang dapat menganalisis wilayah berdasarkan data spasial, agar memberikan gambaran kualitas hidup suatu area berdasarkan fasilitas publik, mobilitas, keamanan, dan lingkungan. Aplikasi ini menggabungkan visualisasi peta interaktif, pengolahan data spasial, perhitungan metrik kelayakhunian. sehingga pengguna dapat memperoleh insight komprehensif mengenai suatu wilayah.

1.  Key Concepts in Futurcity

    1. Spatial Data analysis  
       Konsep utama dalam Futurcity adalah analisis spasial yaitu memanfaatkan data geospasial untuk menghitung jumlah fasilitas, jarak fasilitas dari titik analisis, radius cakupan yaitu 500 meter pada 3 lokasi bersamaan. Futurcity menggunakan OpenStreetMap / Overpass API untuk mengambil fasilitas publik dalam format GeoJSON, kemudian memprosesnya pada sisi backend maupun frontend.
    2. Scoring Engine  
       Konsep scoring dikembangkan dalam 4 hal, seperti

       1. Layanan
       2. Mobilitas
       3. Keamanan
       4. Lingkungan

       Setiap sub-scoring ini dihitung dari distribusi dan kedekatan fasilitas, kemudian dikombinasikan menjadi skor 1 - 100. Bobot penilaian ditentukan secara statis berdasarkan prioritas kelayakan huni umum.

    3. Interactive Map Visualization  
       Peta interaktif merupakan inti dari aplikasi Futurcity, yang dimana aplikasi ini menggunakan MapLibre GL (WebGL) untuk rendering peta yang cepat dan mulus, marker fasilitas berdasarkan kategorinya, pin lokasi untuk menganalisis lokasi yang dipilih, dan radius overlay. Selain itu, interaksi seperti drag, zoom, klik fasilitas, dan pencarian lokasi menggunakan autocomplete sehingga menjadi fitur utama dalam UI Futurcity.
    4. Performance Optimization  
       Futurcity menerapkan strategi optimasi performa di dua sisi untuk memastikan aplikasi tetap cepat meski mengolah data besar, yaitu :
       1. Client-side Caching (React Query): Menyimpan hasil pengambilan data fasilitas dan lokasi di memori browser. Hal ini mencegah aplikasi melakukan permintaan berulang ke server untuk data yang sama, sehingga navigasi antar-halaman terasa instan.
       2. Backend Parallelism (Rayon & Tokio): Di sisi server, Futurcity tidak memproses data secara berurutan, melainkan memanfaatkan seluruh inti prosesor (CPU cores) menggunakan Rayon untuk perhitungan skor paralel, serta Tokio untuk menangani banyak permintaan jaringan secara bersamaan (concurrency). Pendekatan ini memangkas waktu tunggu analisis secara signifikan dibandingkan metode konvensional.

1.  Technology Concepts Used in Futurcity

    1.  Frontend Concepts  
        _Frontend Futurcity_ dirancang untuk menghadirkan pengalaman penggunaan yang cepat, interaktif, dan responsif. Pengembangan antarmuka dilakukan dengan memanfaatkan teknologi modern. Berikut beberapa teknologi yang digunakan untuk mendukung tampilan dan fungsinya.

        1. React dan Vite

           _Futurcity_ memakai _React_ untuk membangun tampilan website. Dengan _React_, setiap bagian tampilan seperti tombol, peta, atau panel skor bisa dibuat sebagai komponen terpisah, sehingga lebih mudah dirapikan, dipakai ulang, dan dikembangkan.
           Untuk menjalankan website saat proses pengembangan, _Futurcity_ menggunakan Vite. Vite membuat aplikasi berjalan sangat cepat, perubahan kode langsung terlihat tanpa harus _restart_, dan hasil _build_ akhirnya juga lebih ringan.
           Dengan menggabungkan _React_ dan _Vite_, tampilan _Futurcity_ bisa tetap cepat, responsif, dan nyaman digunakan, meskipun aplikasi memproses data peta yang cukup berat.

        2. TailwindCSS dan shadcn UI

           _Futurcity_ menggunakan TailwindCSS untuk mengatur tampilan aplikasi. Dengan Tailwind, mudah untuk mengatur warna, ukuran, jarak, dan layout dengan cepat tanpa harus menulis CSS panjang. Hal ini membuat desain aplikasi lebih rapi, konsisten, dan mudah diubah kapan saja.
           Selain itu, _Futurcity_ juga menggunakan shaden UI, yaitu kumpulan komponen yang siap pakai seperti tombol, _cards_, dialog, dan input. Dengan shadcn UI, tampilan aplikasi bisa dibuat modern dan profesional tanpa harus membuat semuanya dari awal.

        3. MapLibre GL

           Untuk bagian peta, _Futurcity_ memakai MapLibre GL, yaitu _library_ yang menampilkan peta interaktif dengan performa tinggi. MapLibre dapat memudahkan pengguna untuk memperbesar, memperkecil, menggeser peta, melihat marker fasilitas, dan menampilkan lingkaran radius 500 m. Peta tetap halus dan cepat meskipun banyak data yang ditampilkan.

        4. React Query

           _Futurcity_ menggunakan _React Query_ untuk mengatur data yang diambil dari API. _React Query_ membantu aplikasi menyimpan data sementara (cache), sehingga tidak perlu terus-menerus memanggil API. Jika koneksi lambat atau API gagal, _React Query_ bisa mencoba lagi secara otomatis. Dengan begitu, aplikasi tetap terasa cepat dan data tetap sinkron antara frontend dan backend.

    2.  Backend Concepts

        _Backend Futurcity_ dibangun menggunakan bahasa pemrograman _Rust_, dengan _Axum_ sebagai _framework_ API dan Tokio sebagai _runtime asynchronous_ yang bisa membuat server menangani banyak permintaan secara paralel. Kombinasi teknologi ini membuat _backend_ bekerja sangat cepat dan stabil, terutama ketika mengolah data peta serta menjalankan proses komputasi yang berat.

        Dalam alurnya, backend menerima request dari frontend, kemudian melakukan berbagai tugas seperti menghitung radius fasilitas, memproses data GeoJSON, hingga mengambil data dari Overpass API menggunakan Reqwest. Rust memastikan seluruh proses berjalan efisien, aman, dan bebas dari error memory seperti yang sering terjadi pada bahasa lain.

        Untuk pemrosesan data paralel seperti perhitungan fasilitas atau pembobotan skor, backend memanfaatkan Rayon, yang mempercepat pengolahan data dalam jumlah besar. Sementara itu, Serde digunakan untuk mengubah struktur data Rust ke format JSON dan sebaliknya, sehingga komunikasi data antara backend dan frontend tetap konsisten dan mudah diproses. Seluruh proses berjalan secara asynchronous melalui Tokio, sehingga server tetap responsif dan dapat melayani banyak permintaan sekaligus tanpa bottleneck.

### Technology Stack

1. React  
   React adalah open-source library JavaScript deklaratif, efisien dan fleksibel untuk membangun antarmuka pengguna. React memungkinkan untuk membuat user interface yang kompleks dengan set kode kecil yang terisolasi yang disebut "komponen". Dalam sistem ini, React digunakan sebagai front-end utama untuk membangun UI yang interaktif, cepat, dan responsif. React menampilkan data livability seperti fasilitas publik dan peta wilayah secara dinamis tanpa perlu memuat ulang halaman.

2. Rust  
   Rust adalah bahasa yang dirancang dengan fokus pada keamanan memori (memory safety) dan efisiensi. Rust memadukan performa sekelas C/C++ dengan sistem keamanan yang mencegah terjadinya kesalahan umum. Dalam Futurcity, Rust digunakan pada sisi backend untuk menangani proses pengolahan data spasial yang intensif, termasuk perhitungan skor indikator berdasarkan radius 500 meter dan komparasi tiga lokasi secara paralel.

3. Framework (Axum)  
   Axum dipilih sebagai web framework utama pada backend Futurcity karena memiliki performa tinggi dan integrasi yang erat dengan ekosistem asynchronous Rust. Axum memfasilitasi pembuatan endpoint REST API yang bertugas menerima koordinat lokasi, melakukan perhitungan livability, serta mengirim hasil analisis ke React untuk divisualisasikan. Kemampuan Axum dalam menangani request paralel sangat penting, terutama karena Futurcity menganalisis tiga titik sekaligus.

4. Async runtime (Tokio)  
   Tokio adalah asynchronous runtime yang memungkinkan Rust melakukan banyak tugas secara bersamaan tanpa membuat server lambat. Dalam Futurcity, Tokio menjalankan operasi paralel yaitu menghitung jarak radius untuk tiga titik lokasi. Tanpa Tokio, analisis multi-lokasi seperti di Futurcity akan terasa lebih lambat dan boros sumber daya.

5. Supporting crates

   1. Reqwest
      Reqwest digunakan sebagai HTTP Client untuk mengambil data fasilitas publik dari Overpass API (OpenStreetMap). Library ini mendukung asynchronous request dan bekerja selaras dengan Tokio, sehingga backend Futurcity dapat mengambil data spasial secara aman.

   2. Rayon  
      Rayon adalah library di ekosistem Rust yang memungkinkan pemrosesan data dalam model paralel, yaitu membagi pekerjaan ke banyak thread sehingga perhitungan besar dapat dilakukan lebih cepat. Ketika Futurcity harus menghitung jarak dan mengelompokkan fasilitas dalam jumlah besar, Rayon membantu memecah pekerjaan ke beberapa thread sehingga perhitungan menjadi lebih efisien. Penggunaan Rayon mempercepat proses komparasi tiga lokasi secara signifikan.

   3. Serde  
      Serde adalah framework populer di Rust untuk serialisasi dan deserialisasi data, terutama JSON. Dalam Futurcity, serde mengubah data Rust menjadi JSON agar dapat dikirim dengan mudah ke React melalui REST API, serta memproses data JSON yang diterima dari Overpass API. Kemampuan Serde yang cepat dan aman sangat penting untuk menjaga performa pipline data.

## Source Code and Explanation

Berikut adalah implementasi kode sumber lengkap dari backend Futurcity yang dibangun menggunakan Rust. Kode ini dibagi menjadi beberapa modul untuk menjaga modularitas dan keterbacaan.

### 1. Main Entry Point (`src/main.rs`)

File ini berfungsi sebagai titik masuk aplikasi, mengatur konfigurasi server, routing API, dan mendefinisikan _handler_ utama untuk endpoint `/calculate-score`.

```rust
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
    "Futurcity Backend is running!"
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
```

**Penjelasan:**

- Menggunakan `axum` untuk routing dan `tokio` sebagai runtime asynchronous.
- Fungsi `calculate_score` mengorkestrasi proses: menerima input -> memanggil service Overpass -> memanggil kalkulator skor -> mengembalikan JSON.
- Query Overpass dibuat secara dinamis menggunakan pattern matching (`match category`).

### 2. Data Models (`src/models.rs`)

Mendefinisikan struktur data (Struct) untuk Type Safety dan serialisasi JSON.

```rust
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
    pub locations: Vec<SingleLocationRequest>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SingleLocationRequest {
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
```

**Penjelasan:**

- Menggunakan `serde` (`Serialize`, `Deserialize`) untuk konversi otomatis antara Struct Rust dan format JSON yang dibutuhkan frontend.
- Memisahkan DTO (_Data Transfer Object_) untuk Request dan Response agar kontrak API jelas.

### 3. Overpass API Service (`src/services/overpass.rs`)

Modul ini menangani komunikasi jaringan ke OpenStreetMap (Overpass API).

```rust
use crate::models::{OverpassResponse, OverpassElement};
use reqwest::Client;
use std::error::Error;
use futures::stream::{self, StreamExt};

const OVERPASS_API_URL: &str = "https://overpass-api.de/api/interpreter";

pub struct OverpassService {
    client: Client,
}

impl OverpassService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_facilities(
        &self,
        queries: Vec<(String, String)>,
    ) -> Result<Vec<(String, Vec<OverpassElement>)>, Box<dyn Error + Send + Sync>> {
        let results = stream::iter(queries)
            .map(|(category, query)| {
                let client = self.client.clone();
                async move {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                    let response = client
                        .post(OVERPASS_API_URL)
                        .body(query)
                        .send()
                        .await?;

                    if !response.status().is_success() {
                        return Err(format!("Overpass API error: {}", response.status()).into());
                    }

                    let data: OverpassResponse = response.json().await?;
                    Ok((category, data.elements))
                }
            })
            .buffer_unordered(2)
            .collect::<Vec<Result<(String, Vec<OverpassElement>), Box<dyn Error + Send + Sync>>>>()
            .await;

        let mut facilities = Vec::new();
        for result in results {
            match result {
                Ok(val) => facilities.push(val),
                Err(e) => eprintln!("Error fetching data: {}", e),
            }
        }

        Ok(facilities)
    }
}
```

**Penjelasan:**

- **Declarative Data Flow**: Menggunakan `stream::iter` dikombinasikan dengan `map` untuk mengubah daftar query menjadi serangkaian tugas (_tasks_) secara deklaratif. Kita mendefinisikan transformasi data (dari query string menjadi HTTP request) tanpa perlu menulis loop `for` imperatif yang memblokir.
- **Controlled Concurrency**: Fungsi `buffer_unordered(2)` adalah contoh penerapan _backpressure_ dalam paradigma reaktif. Ini membatasi hanya 2 permintaan jaringan yang berjalan bersamaan untuk mencegah _rate limiting_ dari server Overpass, sambil tetap menjaga efisiensi dibanding proses serial.
- **Monadic Error Handling**: Penggunaan tipe `Result` merepresentasikan error sebagai nilai (_value_), bukan _exception_. Ini memungkinkan error dipropagasikan atau ditangani menggunakan fungsi-fungsi standar seperti `map_err` atau operator `?`, menjadikan alur program lebih terprediksi.
- **Closure & Ownership**: Blok `async move` menangkap variabel `client` ke dalam lingkupnya, memanfaatkan sistem kepemilikan (_ownership_) Rust untuk memastikan keamanan memori saat tugas dijalankan di thread yang berbeda.

### 4. Score Calculator Service (`src/services/score_calculator.rs`)

Ini adalah modul inti yang berisi logika bisnis untuk menghitung jarak, kontribusi fasilitas, dan skor akhir. Modul ini menerapkan pemrosesan paralel.

```rust
use crate::models::{Facility, FacilityCounts, Scores, OverpassElement};
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::services::category_detection::detect_category;

const R: f64 = 6_371_000.0;

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

fn calculate_contribution(distance: f64, category: &str) -> f64 {
    if distance > 500.0 {
        return 0.0;
    }

    let (max_contribution, decay) = match category {
        "health" => (20.0, 0.7),
        "education" => (20.0, 0.8),
        "market" => (20.0, 0.8),
        "transport" => (20.0, 0.9),
        "walkability" => (15.0, 0.8),
        "recreation" => (15.0, 0.7),
        "safety" => (12.0, 0.6),
        "accessibility" => (10.0, 0.8),
        "police" => (20.0, 0.5),
        "religious" => (12.0, 0.7),
        _ => (10.0, 0.8),
    };

    let norm = distance / 500.0;
    let contribution = max_contribution * (1.0 - norm).powf(decay);

    let min_contribution = max_contribution * 0.1;
    contribution.max(min_contribution)
}

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
        + map.get("accessibility").unwrap_or(&0.0)
        + (map.get("health").unwrap_or(&0.0) * 0.5);

    let environment =
        *map.get("recreation").unwrap_or(&0.0);

    let normalize = |v: f64| v.clamp(0.0, 100.0);

    let services_score = normalize(services);
    let mobility_score = normalize(mobility);
    let safety_score = normalize(safety);
    let environment_score = normalize(environment);

    let overall =
        (services_score * 0.30) +
        (mobility_score * 0.25) +
        (safety_score * 0.25) +
        (environment_score * 0.20);

    let scores = Scores {
        services: services_score,
        mobility: mobility_score,
        safety: safety_score,
        environment: environment_score,
        overall: normalize(overall),
    };

    (scores, counts)
}
```

**Penjelasan:**

- **Functional & Parallel:** Fungsi `process_facilities` menggunakan `par_iter()` (Rayon) untuk memproses ribuan data secara paralel. Penggunaan `filter_map` menunjukkan gaya fungsional untuk transformasi data yang bersih.
- **Distance Decay:** Fungsi `calculate_contribution` menerapkan rumus matematika untuk memberikan bobot lebih pada fasilitas yang lebih dekat.
- **Scoring Logic:** Fungsi `calculate_scores` mengagregasi data menggunakan `HashMap` dan `fold` logic (akumulasi) untuk menghasilkan skor kategori.

### 5. Category Detection Service (`src/services/category_detection.rs`)

Modul ini berfungsi sebagai "detektif" yang menganalisis _tags_ (label) dari data mentah OpenStreetMap untuk menentukan kategori fasilitas yang tepat.

```rust
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
```

**Penjelasan:**

- **Pattern Matching:** Menggunakan fitur `match` Rust yang kuat untuk mengecek kombinasi _tags_ secara efisien.
- **Pure Function:** Fungsi `detect_category` bersifat murni (output hanya bergantung pada input), tanpa efek samping.

### 6. Services Module Definition (`src/services/mod.rs`)

File ini berfungsi sebagai "daftar isi" atau titik masuk untuk modul-modul yang ada di dalam folder `services`.

```rust
pub mod overpass;
pub mod score_calculator;
pub mod category_detection;
```

**Penjelasan:**

- **Modularitas:** Dalam Rust, file mod.rs digunakan untuk mengekspos file-file lain (overpass.rs, score_calculator.rs, dll) agar bisa diakses oleh bagian lain aplikasi (seperti main.rs).

### Screenshot

![Alt text](https://media.discordapp.net/attachments/1299802295617196102/1443793151801163878/Screenshot_2025-11-28_062147.png?ex=692a5ca7&is=69290b27&hm=ea2348d37dcd2f46b13ecb63450d6ebecbeb897ed170add82c505f3bee48eb47&=&format=webp&quality=lossless&width=1424&height=800)

![Alt text](https://media.discordapp.net/attachments/1299802295617196102/1443793151318823103/Screenshot_2025-11-28_061716.png?ex=692a5ca7&is=69290b27&hm=c894dba3cdb0d7e21115ea1526454fe54d96b6a7eecbc2472b89a977cf6ed167&=&format=webp&quality=lossless&width=1426&height=800)

![Alt text](https://media.discordapp.net/attachments/1299802295617196102/1443793152300421234/Screenshot_2025-11-28_062351.png?ex=692a5ca7&is=69290b27&hm=17d2be8e95b62689d917619808fd5943d25c3b05318b84fbeee32a6e43b44ba1&=&format=webp&quality=lossless&width=1425&height=800)

![Alt text](https://media.discordapp.net/attachments/1299802295617196102/1443793152300421234/Screenshot_2025-11-28_062351.png?ex=692a5ca7&is=69290b27&hm=17d2be8e95b62689d917619808fd5943d25c3b05318b84fbeee32a6e43b44ba1&=&format=webp&quality=lossless&width=1425&height=800)

![Alt text](https://media.discordapp.net/attachments/1299802295617196102/1443793150861770874/Screenshot_2025-11-28_062528.png?ex=692a5ca7&is=69290b27&hm=19caf105cadb27a4784f6efabbf40f303168cd60184a0b9c8976af30c4256f3d&=&format=webp&quality=lossless&width=1425&height=800)

### Conclusion

Futurcity dikembangkan sebagai upaya menghadirkan analisis kualitas lingkungan hunian yang lebih terukur, modern, dan mudah dipahami masyarakat. Dengan menggabungkan Rust di sisi backend serta React dan MapLibre di sisi frontend, aplikasi ini mampu mengolah data spasial dalam jumlah besar dengan tetap menjaga pengalaman pengguna yang ringan dan responsif. Arsitektur yang memisahkan komputasi berat ke server melalui Axum, Tokio, dan Rayon membuat proses perhitungan skor livability pada tiga titik sekaligus dapat dijalankan secara paralel tanpa mengorbankan performa.

Pendekatan functional programming memperkuat desain sistem dengan modul-modul analisis yang lebih terstruktur. Prinsip ini membuat proses perhitungan, transformasi JSON, hingga scoring indikator dapat dilakukan secara deterministik dan lebih aman. Kombinasi React, TailwindCSS, shadcn UI, dan React Query memberikan fondasi bagi antarmuka Futurcity yang cepat, bersih, dan intuitif mulai dari visualisasi peta interaktif, pemilihan lokasi serta pemantauan radius.

Secara keseluruhan, Futurcity membuktikan bahwa integrasi Rust dengan paradigma functional programming menawarkan solusi yang stabil dan efisien untuk analisis spasial berskala besar. Aplikasi ini tidak hanya menampilkan lokasi atau fasilitas sekitar, tetapi juga memberikan gambaran menyeluruh tentang kelayakhunian sebuah wilayah melalui pendekatan yang jelas. Dengan fondasi teknologi yang lebih kuat dan desain modular, Futurcity memiliki potensi besar untuk dikembangkan lebih lanjut sebagai alat evaluasi lingkungan hunian yang akurat dan bermanfaat bagi publik.
