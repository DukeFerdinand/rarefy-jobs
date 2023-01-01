use async_trait::async_trait;
use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
use scraper::html::Html;
use scraper::Selector;
use std::ops::Add;

use crate::api::SavedSearch;
use crate::collector::{CollectorTarget, TargetUrlBuilder};
use crate::crawlers::crawler::{Crawler, CrawlerType};
use crate::utils::get_http_client;
use crate::{error, info};

/// The buyee scraper
pub struct Buyee;

impl Buyee {
    pub async fn get_search_html(search: &SavedSearch) -> Result<String, anyhow::Error> {
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

            Ok(html_body.unwrap())
        } else {
            Err(anyhow::format_err!("Could not create http client!"))
        }
    }
}

#[derive(Debug)]
pub struct Listing {
    pub price: String,
    pub images: Vec<String>,
    pub url: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub found_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl Listing {
    fn new(price: String, url: String) -> Listing {
        Listing {
            price,
            url,
            ..Default::default()
        }
    }

    fn set_images(&mut self, images: Vec<String>) {
        self.images = images
    }
}

impl Default for Listing {
    fn default() -> Self {
        Listing {
            price: String::new(),
            images: Vec::new(),
            url: String::new(),
            start_date: NaiveDate::default(),
            // this should never be defaulted, but just in case :)
            end_date: NaiveDate::default().add(Duration::days(2)),
            found_at: Utc::now().date_naive(),
            updated_at: Utc::now().date_naive(),
        }
    }
}

#[async_trait]
impl Crawler for Buyee {
    type Output = Listing;

    fn new() -> Self {
        Self
    }

    fn crawler_type(&self) -> CrawlerType {
        CrawlerType::Buyee
    }

    async fn scrape(&self, search: &SavedSearch) -> Result<Vec<Self::Output>, anyhow::Error> {
        let search_page = Buyee::get_search_html(&search).await?;
        let search_document = Html::parse_document(&search_page);

        // Get the initial item cards (AKA search results)
        let card_selector = Selector::parse("div.itemCard__item").unwrap();

        let mut cards = Vec::new();
        for card in search_document.select(&card_selector) {
            cards.push(card);
        }

        // Nothing to do if the length is zero :(
        if cards.len() == 0 {
            return Ok(Vec::new());
        }

        // Take the selected cards and convert into Listing instances
        let mut listings: Vec<Listing> = Vec::new();
        for card in cards {
            let price: String = card
                // Select to get iterator of elementref
                .select(&Selector::parse(".g-price__outer > span.g-price").unwrap())
                // fold into string using e.text()
                .fold(String::new(), |acc, e| {
                    format!("{}{}", acc, e.text().collect::<Vec<&str>>().join(""))
                });

            let url: String = card
                .select(&Selector::parse(".itemCard__itemName > a").unwrap())
                // Accumulator should never be accessed...
                .fold(String::new(), |_, e| {
                    format!("{}", e.value().attr("href").unwrap())
                });

            // Create initial listing object
            let mut listing = Listing::new(price, url);

            // tokio::task::spawn(async {
            //     //
            // });

            // Push it onto listing stack
            listings.push(listing);
        }

        info!("{:?}", listings);

        Ok(listings)
    }

    async fn process(&self) -> Result<Self::ProcessOutput, anyhow::Error> {
        todo!()
    }
}
