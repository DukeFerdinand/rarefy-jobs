use reqwest::header::{HeaderMap, HeaderValue};

const USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0";

pub fn get_http_client() -> anyhow::Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.append("User-Agent", HeaderValue::from_static(USER_AGENT));

    Ok(reqwest::ClientBuilder::new()
        .brotli(true)
        .default_headers(headers)
        .build()?)
}
