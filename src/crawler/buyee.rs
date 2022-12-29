extern crate reqwest;

use crate::info;

const BUYEE_URL: &str = "https://buyee.jp/item/search/query/{{term}}/category/22260";

pub async fn get_buyee_html(term: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = BUYEE_URL.replace("{{term}}", term);

    info!("{}", url);

    let resp = client.get(BUYEE_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36")
        .send().await?;

    Ok(resp.text().await?)
}