#![feature(async_closure)]

extern crate tokio;
use crawlers::rare_finder::RareFinder;

#[tokio::main]
async fn main() {
    let mut rare_finder = RareFinder::new();

    rare_finder.run();
}
