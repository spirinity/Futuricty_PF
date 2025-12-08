use crate::models::{OverpassResponse, OverpassElement};
use reqwest::Client;
use std::error::Error;
use futures::stream::{self, StreamExt};

const OVERPASS_API_URL: &str = "https://overpass-api.de/api/interpreter";
const RATE_LIMIT_DELAY_MS: u64 = 1000; 
const MAX_CONCURRENT_REQUESTS: usize = 2;
const MAX_RETRIES_PER_CATEGORY: u32 = 3;
const RETRY_DELAY_MS: u64 = 2000;

async fn rate_limit_delay() {
    tokio::time::sleep(std::time::Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
}

async fn fetch_overpass_data_with_retry(
    client: &Client,
    category: String,
    query: String,
) -> Result<(String, Vec<OverpassElement>), Box<dyn Error + Send + Sync>> {
    for attempt in 0..MAX_RETRIES_PER_CATEGORY {
        if attempt > 0 {
            let delay = RETRY_DELAY_MS * (attempt as u64);
            eprintln!("Retrying category '{}' (attempt {}/{}) after {}ms...", 
                category, attempt + 1, MAX_RETRIES_PER_CATEGORY, delay);
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        }
        
        rate_limit_delay().await;
        
        let result = client
            .post(OVERPASS_API_URL)
            .body(query.clone())
            .send()
            .await;
            
        match result {
            Ok(response) => {
                let status = response.status();
                match response.error_for_status() {
                    Ok(resp) => {
                        match resp.json::<OverpassResponse>().await {
                            Ok(data) => {
                                if attempt > 0 {
                                    println!("✓ Category '{}' succeeded on attempt {}", category, attempt + 1);
                                }
                                return Ok((category, data.elements));
                            },
                            Err(e) => {
                                if attempt + 1 >= MAX_RETRIES_PER_CATEGORY {
                                    return Err(format!("JSON parse error: {}", e).into());
                                }
                            }
                        }
                    },
                    Err(e) => {
                        let should_retry = status.as_u16() == 429 || status.as_u16() == 504;
                        if !should_retry || attempt + 1 >= MAX_RETRIES_PER_CATEGORY {
                            return Err(format!("HTTP {}: {}", status, e).into());
                        }
                    }   
                }
            },
            Err(e) => {
                if attempt + 1 >= MAX_RETRIES_PER_CATEGORY {
                    return Err(format!("Request error: {}", e).into());
                }
            }
        }
    }
    
    Err(format!("Category '{}' failed after {} attempts", 
        category, MAX_RETRIES_PER_CATEGORY).into())
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
                    fetch_overpass_data_with_retry(&client, category, query).await
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
