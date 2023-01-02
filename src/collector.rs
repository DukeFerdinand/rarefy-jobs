use crate::api::Api;
use crate::crawlers::buyee::Buyee;
use crate::crawlers::crawler::Crawler;
use crate::info;
use std::default::Default;

pub struct TargetUrlBuilder {
    target: CollectorTarget,

    // builder options
    // vinyl_only: bool,

    // the output
    url: String,
}

impl TargetUrlBuilder {
    pub fn new(target_platform: CollectorTarget) -> TargetUrlBuilder {
        TargetUrlBuilder {
            target: target_platform,

            // builder options
            // vinyl_only: true,
            url: String::new(),
        }
    }

    /// Initialize the base_url
    pub fn base_url(mut self) -> TargetUrlBuilder {
        self.url = String::from(self.target.base_url());

        self
    }

    /// Set the term to search for
    pub fn with_term(mut self, query: &str) -> TargetUrlBuilder {
        match self.target {
            CollectorTarget::Buyee => self.url = format!("{}{}", self.url, query),
        }

        self
    }

    /// Set category to vinyl only
    pub fn vinyl_only(mut self, vinyl_only: bool) -> TargetUrlBuilder {
        match self.target {
            CollectorTarget::Buyee => {
                // Buyee only needs an additional category if the vinyl flag is chosen
                if vinyl_only {
                    self.url = format!("{}{}", self.url, "/category/22260")
                }
            }
        }

        self
    }

    /// Add some anti-scraper-detection measures to url (optional)
    pub fn add_scrape_protection(mut self) -> TargetUrlBuilder {
        match self.target {
            CollectorTarget::Buyee => self.url = format!("{}?suggest=1", self.url),
        }

        self
    }

    pub fn build(self) -> String {
        self.url
    }
}

pub enum CollectorTarget {
    Buyee,
}

impl CollectorTarget {
    /// Get the base URL (no filters) for the target
    pub fn base_url(&self) -> &'static str {
        match &self {
            // https://buyee.jp/item/search/query/{{term}}<query-params-here>
            CollectorTarget::Buyee => "https://buyee.jp/item/search/query/",
        }
    }
}

/// The struct for managing the actual scraping of websites
pub struct Collector {
    targets: [CollectorTarget; 1],
    api: Api,
}

impl Default for Collector {
    fn default() -> Self {
        Self {
            targets: [CollectorTarget::Buyee],
            api: Api::new(),
        }
    }
}

impl Collector {
    /// Get a new collector instance
    pub fn new() -> Collector {
        Collector {
            ..Default::default()
        }
    }

    /// This will run all of the scrape targets.
    /// TODO: Make it only process a passed term
    pub async fn collect_from_all(&self) -> anyhow::Result<()> {
        for target in &self.targets {
            match target {
                CollectorTarget::Buyee => {
                    let saved_searches = self.api.get_saved_searches().await?;
                    for search in saved_searches {
                        info!("> crawling for {}", search.query);

                        let results = Buyee::new().scrape(&search).await?;
                        info!("-> Found {} listings for {}", results.len(), search.query);
                        for listing in results {
                            // These are sort of like goroutines as far as I can tell...
                            // Should not incur massive overhead from doing this?
                            tokio::task::spawn(async move {
                                info!("-> Visiting {}", listing.url);
                                let final_listing = Buyee::new().process(listing).await;
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
