use crate::prisma;
use crate::prisma::{saved_search};
use crate::prisma::saved_search::Data as PrismaSavedSearch;

use crate::info;

enum CrawlerJobType {
    Buyee,
}

pub type SavedSearch = PrismaSavedSearch;

pub async fn find_jobs() -> Result<Vec<SavedSearch>, Box<dyn std::error::Error>> {
    let client = prisma::new_client().await?;

    let saved_searches = client.saved_search().find_many(vec![
        saved_search::search_result::every(vec![]),
    ])
        .exec()
        .await?;

    info!("Found {} saved searches", saved_searches.len());

    // for saved in saved_searches {
    //     if let Some(existing) = saved.search_result {
    //         info!("Found some pre-existing search results, HANDLE THIS for dedupe!");
    //     }
    // }

    Ok(saved_searches)
}