use crate::api::SavedSearch;
use crate::collector::{CollectorTarget, TargetUrlBuilder};
use async_trait::async_trait;

use crate::crawlers::crawler::{Crawler, CrawlerType};
use crate::utils::get_http_client;
use crate::{error, info};

/// The buyee scraper
pub struct Buyee {
    vinyl_only: bool,
}

#[async_trait]
impl Crawler for Buyee {
    type Output = ();

    fn new() -> Self {
        Self { vinyl_only: false }
    }

    fn crawler_type(&self) -> CrawlerType {
        CrawlerType::Buyee
    }

    async fn scrape(&self, search: &SavedSearch) -> Result<Vec<Self::Output>, anyhow::Error> {
        // Create the HTTP client with proper headers
        let fetch_client = get_http_client();

        if let Ok(fetch) = fetch_client {
            // Build the URL based on the current target
            let url = TargetUrlBuilder::new(CollectorTarget::Buyee)
                .base_url()
                .with_term(&search.query)
                .vinyl_only(search.vinyl_only)
                .add_scrape_protection()
                .build();

            // Get the HTML
            let response = fetch.get(url).send().await;

            // Something happened when making a request to Buyee
            if let Err(e) = response {
                error!("Got status code {}", e.status().unwrap());
                // error!("Got error when fetching buyee HTML: {}", e);
            }

            let html_body = response?.text().await;

            // Something happened when trying to get the HTML body from the response object
            if let Err(_) = html_body {
                error!(
                    "Could not parse text body for {}, skipping :(",
                    search.query
                )
            }

            // All is well! Scrape on
            let html = html_body.unwrap();
            info!("Got html with length {}", html.len());

            Ok(Vec::new())
        } else {
            Err(anyhow::format_err!("Could not create http client!"))
        }
    }

    async fn process(&self) -> Result<Self::ProcessOutput, anyhow::Error> {
        todo!()
    }
}
