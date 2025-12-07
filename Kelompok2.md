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

#[macro_export]
macro_rules! serde_clone {
    ($($body:tt)*) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        $($body)*
    };
}

#[macro_export]
macro_rules! serde_clone_default {
    ($($body:tt)*) => {
        #[derive(Debug, Deserialize, Serialize, Clone, Default)]
        $($body)*
    };
}

#[macro_export]
macro_rules! serde_only {
    ($($body:tt)*) => {
        #[derive(Debug, Deserialize, Serialize)]
        $($body)*
    };
}

serde_clone!(
pub struct LocationData {
    pub address: String,
    pub facility_counts: FacilityCounts,
    pub scores: Scores,
    pub nearby_facilities: Vec<String>,
    pub facilities: Vec<Facility>,
}
);

serde_clone_default!(
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
);

serde_clone_default!(
pub struct Scores {
    pub overall: f64,
    pub services: f64,
    pub mobility: f64,
    pub safety: f64,
    pub environment: f64,
}
);

serde_clone!(
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
);

serde_only!(
pub struct CalculateScoreRequest {
    pub locations: Vec<SingleLocationRequest>,
}
);

serde_clone!(
pub struct SingleLocationRequest {
    pub lat: f64,
    pub lng: f64,
}
);

serde_only!(
pub struct OverpassResponse {
    pub elements: Vec<OverpassElement>,
}
);

serde_clone!(
pub struct OverpassElement {
    pub id: u64,
    #[serde(rename = "type")]
    pub element_type: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub center: Option<Center>,
    pub tags: Option<HashMap<String, String>>,
}
);

serde_clone!(
pub struct Center {
    pub lat: f64,
    pub lon: f64,
}
);
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
const RATE_LIMIT_DELAY_MS: u64 = 500;
const MAX_CONCURRENT_REQUESTS: usize = 2;

async fn rate_limit_delay() {
    tokio::time::sleep(std::time::Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
}

async fn fetch_overpass_data(
    client: &Client,
    category: String,
    query: String,
) -> Result<(String, Vec<OverpassElement>), Box<dyn Error + Send + Sync>> {
    rate_limit_delay().await;

    client
        .post(OVERPASS_API_URL)
        .body(query)
        .send()
        .await?
        .error_for_status()
        .map_err(|e| format!("Overpass API error: {}", e))?
        .json::<OverpassResponse>()
        .await
        .map(|data| (category, data.elements))
        .map_err(|e| e.into())
}

#[derive(Clone)]
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
        let facilities = stream::iter(queries)
            .map(|(category, query)| {
                let client = self.client.clone();
                async move {
                    fetch_overpass_data(&client, category, query).await
                }
            })
            .buffer_unordered(MAX_CONCURRENT_REQUESTS)
            .collect::<Vec<Result<(String, Vec<OverpassElement>), Box<dyn Error + Send + Sync>>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<(String, Vec<OverpassElement>)>, Box<dyn Error + Send + Sync>>>()?;
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
```

**Penjelasan:**

- **Functional & Parallel:** Fungsi `process_facilities` menggunakan `par_iter()` (Rayon) untuk memproses ribuan data secara paralel. Penggunaan `filter_map` menunjukkan gaya fungsional untuk transformasi data yang bersih.
- **Distance Decay:** Fungsi `calculate_contribution` menerapkan rumus matematika untuk memberikan bobot lebih pada fasilitas yang lebih dekat.
- **Scoring Logic:** Fungsi `calculate_scores` mengagregasi data menggunakan `HashMap` dan `fold` logic (akumulasi) untuk menghasilkan skor kategori.

### 5. Category Detection Service (`src/services/category_detection.rs`)

Modul ini berfungsi sebagai "detektif" yang menganalisis _tags_ (label) dari data mentah OpenStreetMap untuk menentukan kategori fasilitas yang tepat.

```rust
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

### 7. Config Category Patterns (`config/category_patterns.json`)

File ini berfungsi sebagai..........

```json
{
  "education": {
    "amenity_equals": [
      "school",
      "university",
      "college",
      "kindergarten",
      "library"
    ],
    "name_contains": [
      "sekolah",
      "sma",
      "smp",
      "sd ",
      "smk",
      "universitas",
      "univ",
      "kampus",
      "tk",
      "paud",
      "perpustakaan",
      "library"
    ]
  },
  "police": {
    "amenity_equals": ["police"],
    "name_contains": [
      "polisi",
      "polres",
      "polsek",
      "polda",
      "satlantas",
      "satpol",
      "police"
    ]
  },
  "market": {
    "amenity_equals": [
      "restaurant",
      "cafe",
      "fast_food",
      "food_court",
      "bar",
      "pub",
      "ice_cream",
      "coffee_shop",
      "fuel",
      "gas_station",
      "petrol_station",
      "service_station"
    ],
    "name_contains": [
      "spbu",
      "pom bensin",
      "gas station",
      "pertamina",
      "shell",
      "warung",
      "toko",
      "shop",
      "store",
      "market",
      "mall",
      "plaza"
    ],
    "shop_non_empty": true
  },
  "health": {
    "amenity_equals": [
      "hospital",
      "clinic",
      "doctors",
      "dentist",
      "pharmacy",
      "veterinary"
    ],
    "name_contains": [
      "rumah sakit",
      "rsud",
      "klinik",
      "apotek",
      "dokter",
      "puskesmas",
      "poli"
    ],
    "name_prefixes": ["rs "],
    "name_prefix_exclude": ["sekolah"]
  },
  "transport": {
    "public_transport_equals": ["platform", "station", "stop_position"],
    "highway_equals": ["bus_stop"],
    "railway_equals": ["station", "halt", "tram_stop"],
    "name_contains": [
      "halte",
      "bus stop",
      "terminal",
      "stasiun",
      "station",
      "mrt",
      "lrt",
      "angkot"
    ]
  },
  "religious": {
    "amenity_equals": [
      "place_of_worship",
      "mosque",
      "church",
      "temple",
      "synagogue",
      "hindu_temple",
      "buddhist_temple"
    ],
    "name_contains": ["masjid", "gereja", "katedral", "pura", "vihara", "candi"]
  },
  "recreation": {
    "leisure_equals": [
      "park",
      "playground",
      "sports_centre",
      "fitness_centre",
      "swimming_pool",
      "garden"
    ],
    "amenity_equals": ["cinema", "theatre"],
    "name_contains": [
      "taman",
      "gym",
      "fitness",
      "playground",
      "bioskop",
      "cinema",
      "kolam renang"
    ]
  },
  "walkability": {
    "highway_equals": [
      "footway",
      "pedestrian",
      "path",
      "steps",
      "crossing",
      "street_lamp"
    ],
    "route_equals": ["foot", "hiking", "walking"],
    "amenity_equals": ["bench", "drinking_water"],
    "natural_equals": ["tree_row", "hedge"],
    "landuse_equals": ["grass", "meadow"],
    "lit_yes": true,
    "traffic_calming_present": true
  },
  "accessibility": {
    "barrier_equals": ["kerb"],
    "kerb_equals": ["lowered", "flush"],
    "highway_equals": ["elevator"],
    "wheelchair_equals": ["yes"],
    "amenity_equals": ["toilets"],
    "tactile_paving_yes": true
  },
  "safety": {
    "highway_equals": ["street_lamp"],
    "lit_yes": true,
    "traffic_calming_present": true,
    "man_made_equals": ["surveillance"],
    "amenity_equals": ["fire_station", "hospital"]
  }
}
```

**Penjelasan:**

- **.....:** Dalam json, file ini digunakan untuk .......

### 8. Config Queries (`config/queries.json`)

File ini berfungsi sebagai..........

```json
{
  "queries": {
    "health": {
      "description": "Fasilitas kesehatan: rumah sakit, klinik, apotek, dokter, puskesmas",
      "node_amenity": "hospital|clinic|doctors|dentist|pharmacy|veterinary",
      "way_amenity": "hospital|clinic|doctors|dentist|pharmacy|veterinary",
      "node_name": "rumah sakit|rsud|klinik|apotek|apotik|dokter|puskesmas|poli",
      "way_name": "rumah sakit|rsud|klinik|apotek|apotik|dokter|puskesmas|poli"
    },
    "education": {
      "description": "Fasilitas pendidikan: sekolah, universitas, perpustakaan",
      "node_amenity": "school|university|college|kindergarten|library",
      "way_amenity": "school|university|college|kindergarten|library",
      "node_name": "sekolah|sd|smp|sma|smk|universitas|univ|kampus|tk|paud|perpustakaan|library",
      "way_name": "sekolah|sd|smp|sma|smk|universitas|univ|kampus|tk|paud|perpustakaan|library"
    },
    "market": {
      "description": "Fasilitas belanja dan kuliner: toko, restoran, cafe, SPBU",
      "node_shop": ".*",
      "way_shop": ".*",
      "node_amenity": "restaurant|cafe|fast_food|food_court|bar|pub|ice_cream|coffee_shop|shop|store|market|retail|food|beverage|fuel|gas_station|petrol_station|service_station",
      "way_amenity": "restaurant|cafe|fast_food|food_court|bar|pub|ice_cream|coffee_shop|shop|store|market|retail|food|beverage|fuel|gas_station|petrol_station|service_station",
      "node_name": "spbu|pom bensin|gas station|petrol|fuel|bensin|solar|pertamina|shell|bp|esso|caltex|toko|warung|shop|store|market|mall|plaza",
      "way_name": "spbu|pom bensin|gas station|petrol|fuel|bensin|solar|pertamina|shell|bp|esso|caltex|toko|warung|shop|store|market|mall|plaza"
    },
    "transport": {
      "description": "Transportasi publik: halte, stasiun, terminal",
      "node_public_transport": "platform|station|stop_position",
      "way_public_transport": "platform|station",
      "node_highway": "bus_stop",
      "node_railway": "station|halt|tram_stop",
      "node_name": "halte|bus stop|terminal|stasiun|station|mrt|lrt|transjakarta|angkot",
      "way_name": "halte|bus stop|terminal|stasiun|station|mrt|lrt|transjakarta|angkot"
    },
    "walkability": {
      "description": "Infrastruktur pejalan kaki: trotoar, crossing, pedestrian zone",
      "way_highway": "footway|pedestrian|path|steps|bridleway",
      "way_sidewalk": "both|left|right|separate",
      "way_footway": "sidewalk|crossing",
      "node_highway": "crossing|street_lamp",
      "way_pedestrian": "yes|designated|zone",
      "way_route": "foot|hiking|walking",
      "node_amenity": "bench|drinking_water",
      "way_amenity": "bench|drinking_water",
      "way_lit": "yes"
    },
    "recreation": {
      "description": "Fasilitas rekreasi: taman, playground, kolam renang, gym",
      "node_leisure": "park|playground|sports_centre|fitness_centre|swimming_pool|garden",
      "way_leisure": "park|playground|sports_centre|fitness_centre|swimming_pool|garden",
      "node_amenity": "cinema|theatre",
      "way_amenity": "cinema|theatre",
      "node_name": "taman|park|playground|kolam renang|swimming|gym|fitness|bioskop|cinema|teater|theatre",
      "way_name": "taman|park|playground|kolam renang|swimming|gym|fitness|bioskop|cinema|teater|theatre"
    },
    "safety": {
      "description": "Keamanan dan pencegahan: lampu jalan, crossing, polisi, pemadam",
      "node_highway": "street_lamp|crossing|traffic_signals",
      "way_highway": "street_lamp",
      "way_traffic_calming": "yes",
      "way_sidewalk": "yes",
      "node_amenity": "fire_station|hospital",
      "way_amenity": "fire_station|hospital",
      "node_man_made": "surveillance",
      "way_man_made": "surveillance",
      "way_lit": "yes"
    },
    "police": {
      "description": "Kepolisian: kantor polisi, polres, polsek",
      "node_amenity": "police",
      "way_amenity": "police",
      "node_name": "polisi|polres|polsek|polda|satlantas|satpol|pp|police",
      "way_name": "polisi|polres|polsek|polda|satlantas|satpol|pp|police"
    },
    "religious": {
      "description": "Fasilitas keagamaan: masjid, gereja, kuil, vihara",
      "node_amenity": "place_of_worship|mosque|church|temple|synagogue|hindu_temple|buddhist_temple",
      "way_amenity": "place_of_worship|mosque|church|temple|synagogue|hindu_temple|buddhist_temple",
      "node_name": "masjid|gudang|gereja|katedral|synagogue|hindu_temple|buddhist_temple",
      "way_name": "masjid|gudang|gereja|katedral|synagogue|hindu_temple|buddhist_temple"
    },
    "accessibility": {
      "description": "Aksesibilitas penyandang disabilitas: kerb rendah, elevator, toilet",
      "node_barrier": "kerb",
      "way_barrier": "kerb",
      "node_kerb": "lowered",
      "way_kerb": "lowered",
      "node_highway": "elevator",
      "way_highway": "elevator",
      "node_amenity": "parking|toilets",
      "way_amenity": "parking|toilets",
      "node_tactile_paving": "yes",
      "way_tactile_paving": "yes",
      "node_wheelchair": "yes",
      "way_wheelchair": "yes"
    }
  },
  "settings": {
    "default_distance": 500,
    "output_format": "json",
    "output_type": "center"
  }
}
```

**Penjelasan:**

- **.....:** Dalam json, file ini digunakan untuk .......

### 9. Config Scoring (`config/scoring_config.json`)

File ini berfungsi sebagai..........

```json
{
  "contribution_weights": {
    "health": {
      "max_contribution": 20.0,
      "decay_factor": 0.7,
      "min_contribution_ratio": 0.1
    },
    "education": {
      "max_contribution": 20.0,
      "decay_factor": 0.8,
      "min_contribution_ratio": 0.1
    },
    "market": {
      "max_contribution": 20.0,
      "decay_factor": 0.8,
      "min_contribution_ratio": 0.1
    },
    "transport": {
      "max_contribution": 20.0,
      "decay_factor": 0.9,
      "min_contribution_ratio": 0.1
    },
    "walkability": {
      "max_contribution": 15.0,
      "decay_factor": 0.8,
      "min_contribution_ratio": 0.1
    },
    "recreation": {
      "max_contribution": 15.0,
      "decay_factor": 0.7,
      "min_contribution_ratio": 0.1
    },
    "safety": {
      "max_contribution": 12.0,
      "decay_factor": 0.6,
      "min_contribution_ratio": 0.1
    },
    "accessibility": {
      "max_contribution": 10.0,
      "decay_factor": 0.8,
      "min_contribution_ratio": 0.1
    },
    "police": {
      "max_contribution": 20.0,
      "decay_factor": 0.5,
      "min_contribution_ratio": 0.1
    },
    "religious": {
      "max_contribution": 12.0,
      "decay_factor": 0.7,
      "min_contribution_ratio": 0.1
    },
    "default": {
      "max_contribution": 10.0,
      "decay_factor": 0.8,
      "min_contribution_ratio": 0.1
    }
  },
  "score_weights": {
    "services_weight": 0.3,
    "mobility_weight": 0.25,
    "safety_weight": 0.25,
    "environment_weight": 0.2,
    "health_contribution_to_safety": 0.5,
    "score_clamp_min": 0.0,
    "score_clamp_max": 100.0
  },
  "category_mappings": {
    "services": ["health", "education", "market", "religious"],
    "mobility": ["transport", "walkability"],
    "safety": ["safety", "police", "accessibility"],
    "environment": ["recreation"]
  },
  "name_extraction": {
    "fallback_fields": ["name", "amenity", "shop", "leisure", "highway"],
    "default_name": "facility"
  }
}
```

**Penjelasan:**

- **.....:** Dalam json, file ini digunakan untuk .......

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
