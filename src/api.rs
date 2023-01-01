use crate::info;
use serde::{Deserialize, Serialize};

pub struct Api {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearch {
    pub id: String,
    pub name: String,
    pub query: String,
    pub created_at: String,
    pub updated_at: String,
    pub vinyl_only: bool,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct SavedSearchResponse {
    searches: Vec<SavedSearch>,
}

impl Api {
    pub fn new() -> Api {
        let base_url = std::env::var("API_HOST").unwrap();

        if let Ok(fetch_client) = reqwest::ClientBuilder::new().build() {
            Api {
                base_url,
                client: fetch_client,
            }
        } else {
            panic!("Could not create API client, please handle this more gracefully")
        }
    }

    fn fmt_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn get_saved_searches(&self) -> anyhow::Result<Vec<SavedSearch>> {
        let url = self.fmt_url("/api/saved_searches");
        let res = self.client.get(url).send().await?;

        let status = res.status().as_u16();
        match status {
            200 => {
                let searches = res.json::<SavedSearchResponse>().await?.searches;

                info!("Found {} saved searches", searches.len());
                return Ok(searches);
            }
            _ => Err(anyhow::anyhow!("Got status: {}", status)),
        }
    }
}
