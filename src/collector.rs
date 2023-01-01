use crate::api::Api;
use crate::info;
use std::default::Default;

pub enum CollectorTarget {
    Buyee,
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
                        info!("> crawling for {}", search.query)
                    }
                }
            }
        }

        Ok(())
    }
}
