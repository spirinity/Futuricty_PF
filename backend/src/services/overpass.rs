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
