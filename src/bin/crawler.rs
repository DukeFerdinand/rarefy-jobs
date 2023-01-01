#![feature(async_closure)]

extern crate tokio;
use crawlers::scraper::Scraper;

#[tokio::main]
async fn main() {
    let mut scraper = Scraper::new();

    scraper.run();
}
