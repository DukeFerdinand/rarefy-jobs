use crate::crawler::buyee::get_buyee_html;
use crate::crawler::find_jobs::SavedSearch;

pub async fn scrape_data(job: SavedSearch) -> Result<(), Box<dyn std::error::Error>> {
    let result: String = tokio::spawn(async move {
        let result = get_buyee_html(&job.query).await;

        return "".to_string()
    }).await?;



    Ok(())
}