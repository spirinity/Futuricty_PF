use crate::models::{OverpassResponse, OverpassElement};
use reqwest::Client;
use std::error::Error;
use futures::stream::{self, StreamExt};

const OVERPASS_API_URL: &str = "https://overpass-api.de/api/interpreter";
const RATE_LIMIT_DELAY_MS: u64 = 500;  // Delay antar request untuk rate limiting (milliseconds)

/// Pure function untuk rate limiting delay
/// Reusable untuk berbagai API calls yang membutuhkan rate limiting
async fn rate_limit_delay() {
    tokio::time::sleep(std::time::Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
}

/// Pure function untuk fetch data dari Overpass API dengan functional error handling
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
        .error_for_status()  // Functional: auto-convert non-success to Err
        .map_err(|e| format!("Overpass API error: {}", e))?
        .json::<OverpassResponse>()
        .await
        .map(|data| (category, data.elements))  // Functional transform
        .map_err(|e| e.into())
}

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
            .buffer_unordered(2)
            .collect::<Vec<Result<(String, Vec<OverpassElement>), Box<dyn Error + Send + Sync>>>>()
            .await
            .into_iter()
            .filter_map(|result| result.map_err(|e| eprintln!("Error fetching data: {}", e)).ok())
            .collect();

        Ok(facilities)
    }
}
