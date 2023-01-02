use crate::api::SavedSearch;
use async_trait::async_trait;

pub enum CrawlerType {
    Buyee,
}

pub type ErrorVec = Vec<String>;

#[async_trait]
pub trait Crawler {
    type Output;
    type ProcessOutput = ();

    fn new() -> Self;
    fn crawler_type(&self) -> CrawlerType;
    async fn scrape(&self, search: &SavedSearch) -> Result<Vec<Self::Output>, anyhow::Error>;
    async fn process(&self, listings: Self::Output) -> Result<Self::ProcessOutput, anyhow::Error>;
}
