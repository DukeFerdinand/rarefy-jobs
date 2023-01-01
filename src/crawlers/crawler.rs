use crate::api::SavedSearch;
use async_trait::async_trait;

pub enum CrawlerType {
    Buyee,
}

pub type ErrorVec = Vec<String>;

#[async_trait]
pub trait Crawler: Send + Sync {
    type Output;
    type ProcessOutput = ();

    fn new() -> Self;
    fn crawler_type(&self) -> CrawlerType;
    async fn scrape(&self, search: &SavedSearch) -> Result<Vec<Self::Output>, anyhow::Error>;
    async fn process(&self) -> Result<Self::ProcessOutput, anyhow::Error>;
}
