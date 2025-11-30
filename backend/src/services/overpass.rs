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

        let facilities = results.into_iter()
            .filter_map(|result| {
                match result {
                    Ok(val) => Some(val),
                    Err(e) => {
                        eprintln!("Error fetching data: {}", e);
                        None
                    }
                }
            })
            .collect();

        Ok(facilities)
    }
}
